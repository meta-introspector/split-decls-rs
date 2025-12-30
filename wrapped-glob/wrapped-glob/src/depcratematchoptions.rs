// Generated macro for MatchOptions (struct)
macro_rules! DepcrateMatchOptions {
() => {
// Module: crate
// Provides: {"MatchOptions"}
// Dependencies: {}
# [doc = " Configuration options to modify the behaviour of `Pattern::matches_with(..)`."] # [allow (missing_copy_implementations)] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Default)] pub struct MatchOptions { # [doc = " Whether or not patterns should be matched in a case-sensitive manner."] # [doc = " This currently only considers upper/lower case relationships between"] # [doc = " ASCII characters, but in future this might be extended to work with"] # [doc = " Unicode."] pub case_sensitive : bool , # [doc = " Whether or not path-component separator characters (e.g. `/` on"] # [doc = " Posix) must be matched by a literal `/`, rather than by `*` or `?` or"] # [doc = " `[...]`."] pub require_literal_separator : bool , # [doc = " Whether or not paths that contain components that start with a `.`"] # [doc = " will require that `.` appears literally in the pattern; `*`, `?`, `**`,"] # [doc = " or `[...]` will not match. This is useful because such files are"] # [doc = " conventionally considered hidden on Unix systems and it might be"] # [doc = " desirable to skip them when listing files."] pub require_literal_leading_dot : bool , }
};
}
