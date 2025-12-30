// Generated macro for gather_stats (function)
macro_rules! Depcrate_outputgather_stats {
() => {
// Module: crate::output
// Provides: {"gather_stats"}
// Dependencies: {}
# [doc = " Generate a short list of occurring lints-types and their count"] fn gather_stats (warnings : & [ClippyWarning]) -> (String , HashMap < & String , usize >) { let mut counter : HashMap < & String , usize > = HashMap :: new () ; for wrn in warnings { * counter . entry (& wrn . name) . or_insert (0) += 1 ; } let mut stats : Vec < (& & String , & usize) > = counter . iter () . collect () ; stats . sort_by_key (| (lint , count) | format ! ("{count:0>4}, {lint}")) ; let mut header = String :: from ("| lint                                               | count |\n") ; header . push_str ("| -------------------------------------------------- | ----- |\n") ; let stats_string = stats . iter () . map (| (lint , count) | format ! ("| {lint:<50} |  {count:>4} |\n")) . fold (header , | mut table , line | { table . push_str (& line) ; table }) ; (stats_string , counter) }
};
}
