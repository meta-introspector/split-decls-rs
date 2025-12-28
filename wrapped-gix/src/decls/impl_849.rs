macro_rules! deps {
    () => {
        Read!();
        Repository!();
        ThreadSafeRepository!();
        Proxy!();
        Error!();
        Note!();
        Path!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        impl Proxy < '_ > { # [doc = " Read the location of the checkout, the base of the work tree."] # [doc = " Note that the location might not exist."] pub fn base (& self) -> std :: io :: Result < PathBuf > { let git_dir = self . git_dir . join ("gitdir") ; let base_dot_git = gix_discover :: path :: from_plain_file (& git_dir) . ok_or_else (| | { std :: io :: Error :: new (std :: io :: ErrorKind :: NotFound , format ! ("Required file '{}' does not exist" , git_dir . display ()) ,) }) ? ? ; Ok (gix_discover :: path :: without_dot_git_dir (base_dot_git)) } # [doc = " The git directory for the work tree, typically contained within the parent git dir."] pub fn git_dir (& self) -> & Path { & self . git_dir } # [doc = " The name of the worktree, which is derived from its folder within the `worktrees` directory within the parent `.git` folder."] pub fn id (& self) -> & BStr { gix_path :: os_str_into_bstr (self . git_dir . file_name () . expect ("worktrees/ parent dir")) . expect ("no illformed UTF-8") } # [doc = " Return true if the worktree cannot be pruned, moved or deleted, which is useful if it is located on an external storage device."] pub fn is_locked (& self) -> bool { self . git_dir . join ("locked") . is_file () } # [doc = " Provide a reason for the locking of this worktree, if it is locked at all."] # [doc = ""] # [doc = " Note that we squelch errors in case the file cannot be read in which case the"] # [doc = " reason is an empty string."] pub fn lock_reason (& self) -> Option < BString > { std :: fs :: read (self . git_dir . join ("locked")) . ok () . map (| contents | contents . trim () . into ()) } # [doc = " Transform this proxy into a [`Repository`] while ignoring issues reading `base()` and ignoring that it might not exist."] # [doc = ""] # [doc = " Most importantly, the `Repository` might be initialized with a non-existing work tree directory as the checkout"] # [doc = " was removed or moved in the mean time or is unavailable for other reasons."] # [doc = " The caller will encounter io errors if it's used like the work tree is guaranteed to be present, but can still access"] # [doc = " a lot of information if work tree access is avoided."] pub fn into_repo_with_possibly_inaccessible_worktree (self) -> Result < Repository , crate :: open :: Error > { let base = self . base () . ok () ; let repo = ThreadSafeRepository :: open_from_paths (self . git_dir , base , self . parent . options . clone ()) ? ; Ok (repo . into ()) } # [doc = " Like `into_repo_with_possibly_inaccessible_worktree()` but will fail if the `base()` cannot be read or"] # [doc = " if the worktree doesn't exist."] # [doc = ""] # [doc = " Note that it won't fail if the worktree doesn't exist."] pub fn into_repo (self) -> Result < Repository , into_repo :: Error > { let base = self . base () ? ; if ! base . is_dir () { return Err (into_repo :: Error :: MissingWorktree { base }) ; } let repo = ThreadSafeRepository :: open_from_paths (self . git_dir , base . into () , self . parent . options . clone ()) ? ; Ok (repo . into ()) } }
    };
}

impl_849!()