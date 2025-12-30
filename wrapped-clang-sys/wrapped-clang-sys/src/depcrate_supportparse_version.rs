// Generated macro for parse_version (function)
macro_rules! Depcrate_supportparse_version {
() => {
// Module: crate::support
// Provides: {"parse_version"}
// Dependencies: {}
# [doc = " Parses the version from the output of a `clang` executable if possible."] fn parse_version (path : & Path) -> Option < CXVersion > { let output = run_clang (path , & ["--version"]) . 0 ; let start = output . find ("version ") ? + 8 ; let mut numbers = output [start ..] . split_whitespace () . next () ? . split ('.') ; let major = numbers . next () . and_then (parse_version_number) ? ; let minor = numbers . next () . and_then (parse_version_number) ? ; let subminor = numbers . next () . and_then (parse_version_number) . unwrap_or (0) ; Some (CXVersion { Major : major , Minor : minor , Subminor : subminor , }) }
};
}
