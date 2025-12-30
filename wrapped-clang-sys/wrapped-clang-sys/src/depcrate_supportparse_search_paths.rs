// Generated macro for parse_search_paths (function)
macro_rules! Depcrate_supportparse_search_paths {
() => {
// Module: crate::support
// Provides: {"parse_search_paths"}
// Dependencies: {}
# [doc = " Parses the search paths from the output of a `clang` executable if possible."] fn parse_search_paths (path : & Path , language : & str , args : & [String]) -> Option < Vec < PathBuf > > { let mut clang_args = vec ! ["-E" , "-x" , language , "-" , "-v"] ; clang_args . extend (args . iter () . map (| s | & * * s)) ; let output = run_clang (path , & clang_args) . 1 ; let start = output . find ("#include <...> search starts here:") ? + 34 ; let end = output . find ("End of search list.") ? ; let paths = output [start .. end] . replace ("(framework directory)" , "") ; Some (paths . lines () . filter (| l | ! l . is_empty ()) . map (| l | Path :: new (l . trim ()) . into ()) . collect () ,) }
};
}
