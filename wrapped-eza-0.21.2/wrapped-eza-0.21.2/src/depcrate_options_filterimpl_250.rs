// Generated macro for impl_250 (impl)
macro_rules! Depcrate_options_filterimpl_250 {
() => {
// Module: crate::options::filter
// Provides: {"impl_250"}
// Dependencies: {}
impl DotFilter { # [doc = " Determines the dot filter based on how many `--all` options were"] # [doc = " given: one will show dotfiles, but two will show `.` and `..` too."] # [doc = " --almost-all is equivalent to --all, included for compatibility with"] # [doc = " `ls -A`."] # [doc = ""] # [doc = " It also checks for the `--tree` option, because of a special case"] # [doc = " where `--tree --all --all` won’t work: listing the parent directory"] # [doc = " in tree mode would loop onto itself!"] # [doc = ""] # [doc = " `--almost-all` binds stronger than multiple `--all` as we currently do not take the order"] # [doc = " of arguments into account and it is the safer option (does not clash with `--tree`)"] pub fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let all_count = matches . count (& flags :: ALL) ; let has_almost_all = matches . has (& flags :: ALMOST_ALL) ? ; match (all_count , has_almost_all) { (0 , false) => Ok (Self :: JustFiles) , (1 , _) | (0 , true) => Ok (Self :: Dotfiles) , (c , _) => { if matches . count (& flags :: TREE) > 0 { Err (OptionsError :: TreeAllAll) } else if matches . is_strict () && c > 2 { Err (OptionsError :: Conflict (& flags :: ALL , & flags :: ALL)) } else { Ok (Self :: DotfilesAndDots) } } } } }
};
}
