macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! assure_no_symlink_in_root {
    () => {
        deps!();
        # [doc = " Note that we only check symlinks on the way from `worktree_root` to `root`,"] # [doc = " so `worktree_root` may go through a symlink."] # [doc = " Returns `(worktree_root, normalized_worktree_relative_root)`."] fn assure_no_symlink_in_root < 'root > (worktree_root : & Path , root : & 'root Path ,) -> Result < (PathBuf , Cow < 'root , Path >) , Error > { let mut current = worktree_root . to_owned () ; let worktree_relative = root . strip_prefix (worktree_root) . expect ("BUG: root was created from worktree_root + prefix") ; let worktree_relative = gix_path :: normalize (worktree_relative . into () , Path :: new ("")) . ok_or (Error :: NormalizeRoot { root : root . to_owned () }) ? ; for (idx , component) in worktree_relative . components () . enumerate () { current . push (component) ; let meta = current . symlink_metadata () . map_err (| err | Error :: SymlinkMetadata { source : err , path : current . to_owned () , }) ? ; if meta . is_symlink () { return Err (Error :: SymlinkInRoot { root : root . to_owned () , worktree_root : worktree_root . to_owned () , component_index : idx , }) ; } } Ok ((current , worktree_relative)) }
    };
}

assure_no_symlink_in_root!()