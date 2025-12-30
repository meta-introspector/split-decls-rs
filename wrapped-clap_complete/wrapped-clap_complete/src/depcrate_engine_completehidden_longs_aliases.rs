// Generated macro for hidden_longs_aliases (function)
macro_rules! Depcrate_engine_completehidden_longs_aliases {
() => {
// Module: crate::engine::complete
// Provides: {"hidden_longs_aliases"}
// Dependencies: {}
# [doc = " Gets all the long hidden aliases and flags of a [`clap::Command`]."] fn hidden_longs_aliases (p : & clap :: Command) -> Vec < CompletionCandidate > { debug ! ("longs: name={}" , p . get_name ()) ; p . get_arguments () . filter_map (| a | { a . get_aliases () . map (| longs | { longs . into_iter () . map (| s | { populate_arg_candidate (CompletionCandidate :: new (format ! ("--{s}")) , a) . hide (true) }) }) }) . flatten () . collect () }
};
}
