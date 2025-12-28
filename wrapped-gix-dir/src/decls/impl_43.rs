macro_rules! deps {
    () => {
        Options!();
        EntryRef!();
        Outcome!();
        Mark!();
        Delegate!();
        State!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl State { # [doc = " Hold the entry with the given `status` if it's a candidate for collapsing the containing directory."] fn held_for_directory_collapse (& mut self , rela_path : & BStr , info : classify :: Outcome , opts : & Options < '_ >) -> bool { if opts . should_hold (info . status) { self . on_hold . push (EntryRef :: from_outcome (Cow :: Borrowed (rela_path) , info) . into_owned ()) ; true } else { false } } # [doc = " Keep track of state we need to later resolve the state."] # [doc = " Top-level directories are special, as they don't fold."] fn mark (& self , may_collapse : bool) -> Mark { Mark { start_index : self . on_hold . len () , may_collapse , } } pub (super) fn new (worktree_root : & Path , current_dir : & Path , is_delete_mode : bool) -> Self { let worktree_relative_current_dir = if is_delete_mode { gix_path :: realpath_opts (worktree_root , current_dir , gix_path :: realpath :: MAX_SYMLINKS) . ok () . and_then (| real_worktree_root | current_dir . strip_prefix (real_worktree_root) . ok () . map (ToOwned :: to_owned)) . map (| relative_cwd | worktree_root . join (relative_cwd)) } else { None } ; Self { on_hold : Vec :: new () , worktree_relative_current_dir , } } # [doc = " Returns `true` if the worktree-relative `directory_to_traverse` is not the current working directory."] # [doc = " This is only the case when"] pub (super) fn may_collapse (& self , directory_to_traverse : & Path) -> bool { self . worktree_relative_current_dir . as_ref () . is_none_or (| cwd | cwd != directory_to_traverse) } pub (super) fn emit_remaining (& mut self , may_collapse : bool , opts : Options < '_ > , out : & mut walk :: Outcome , delegate : & mut dyn walk :: Delegate ,) { if self . on_hold . is_empty () { return ; } _ = Mark { start_index : 0 , may_collapse , } . emit_all_held (self , opts , out , delegate) ; } }
    };
}

impl_43!();