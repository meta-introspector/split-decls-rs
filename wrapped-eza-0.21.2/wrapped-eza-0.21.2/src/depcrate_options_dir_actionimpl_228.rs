// Generated macro for impl_228 (impl)
macro_rules! Depcrate_options_dir_actionimpl_228 {
() => {
// Module: crate::options::dir_action
// Provides: {"impl_228"}
// Dependencies: {}
impl DirAction { # [doc = " Determine which action to perform when trying to list a directory."] # [doc = " There are three possible actions, and they overlap somewhat: the"] # [doc = " `--tree` flag is another form of recursion, so those two are allowed"] # [doc = " to both be present, but the `--list-dirs` flag is used separately."] pub fn deduce (matches : & MatchedFlags < '_ > , can_tree : bool) -> Result < Self , OptionsError > { let recurse = matches . has (& flags :: RECURSE) ? ; let as_file = matches . has (& flags :: LIST_DIRS) ? ; let tree = matches . has (& flags :: TREE) ? ; if matches . is_strict () { if ! recurse && ! tree && matches . count (& flags :: LEVEL) > 0 { return Err (OptionsError :: Useless2 (& flags :: LEVEL , & flags :: RECURSE , & flags :: TREE ,)) ; } else if recurse && as_file { return Err (OptionsError :: Conflict (& flags :: RECURSE , & flags :: LIST_DIRS)) ; } else if tree && as_file { return Err (OptionsError :: Conflict (& flags :: TREE , & flags :: LIST_DIRS)) ; } } if tree && can_tree { Ok (Self :: Recurse (RecurseOptions :: deduce (matches , true) ?)) } else if recurse { Ok (Self :: Recurse (RecurseOptions :: deduce (matches , false) ?)) } else if as_file { Ok (Self :: AsFile) } else { Ok (Self :: List) } } }
};
}
