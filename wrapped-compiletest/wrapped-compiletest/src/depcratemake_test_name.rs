// Generated macro for make_test_name (function)
macro_rules! Depcratemake_test_name {
() => {
// Module: crate
// Provides: {"make_test_name"}
// Dependencies: {}
# [doc = " Creates a name for this test/revision that can be handed over to the executor."] fn make_test_name (config : & Config , testpaths : & TestPaths , revision : Option < & str >) -> String { let path = testpaths . file . strip_prefix (& config . src_root) . unwrap () ; let debugger = match config . debugger { Some (d) => format ! ("-{}" , d) , None => String :: new () , } ; let mode_suffix = match config . compare_mode { Some (ref mode) => format ! (" ({})" , mode . to_str ()) , None => String :: new () , } ; format ! ("[{}{}{}] {}{}" , config . mode , debugger , mode_suffix , path , revision . map_or ("" . to_string () , | rev | format ! ("#{}" , rev))) }
};
}
