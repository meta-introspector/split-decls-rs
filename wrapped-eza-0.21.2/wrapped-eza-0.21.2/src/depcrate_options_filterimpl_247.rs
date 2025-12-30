// Generated macro for impl_247 (impl)
macro_rules! Depcrate_options_filterimpl_247 {
() => {
// Module: crate::options::filter
// Provides: {"impl_247"}
// Dependencies: {}
impl FileFilter { # [doc = " Determines which of all the file filter options to use."] pub fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { use FileFilterFlags as FFF ; let mut filter_flags : Vec < FileFilterFlags > = vec ! [] ; for (has , flag) in & [(matches . has (& flags :: REVERSE) ? , FFF :: Reverse) , (matches . has (& flags :: ONLY_DIRS) ? , FFF :: OnlyDirs) , (matches . has (& flags :: ONLY_FILES) ? , FFF :: OnlyFiles) , (matches . has (& flags :: NO_SYMLINKS) ? , FFF :: NoSymlinks) , (matches . has (& flags :: SHOW_SYMLINKS) ? , FFF :: ShowSymlinks) , (matches . has (& flags :: DIRS_LAST) ? , FFF :: ListDirsLast) , (matches . has (& flags :: DIRS_FIRST) ? , FFF :: ListDirsFirst) ,] { if * has { filter_flags . push (flag . clone ()) ; } } # [rustfmt :: skip] return Ok (Self { no_symlinks : filter_flags . contains (& FFF :: NoSymlinks) , show_symlinks : filter_flags . contains (& FFF :: ShowSymlinks) , flags : filter_flags , sort_field : SortField :: deduce (matches) ? , dot_filter : DotFilter :: deduce (matches) ? , ignore_patterns : IgnorePatterns :: deduce (matches) ? , git_ignore : GitIgnore :: deduce (matches) ? , }) ; } }
};
}
