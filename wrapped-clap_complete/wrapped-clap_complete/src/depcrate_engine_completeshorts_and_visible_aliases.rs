// Generated macro for shorts_and_visible_aliases (function)
macro_rules! Depcrate_engine_completeshorts_and_visible_aliases {
() => {
// Module: crate::engine::complete
// Provides: {"shorts_and_visible_aliases"}
// Dependencies: {}
# [doc = " Gets all the short options, their visible aliases and flags of a [`clap::Command`]."] # [doc = " Includes `h` and `V` depending on the [`clap::Command`] settings."] fn shorts_and_visible_aliases (p : & clap :: Command) -> Vec < CompletionCandidate > { debug ! ("shorts: name={}" , p . get_name ()) ; p . get_arguments () . filter_map (| a | { a . get_short_and_visible_aliases () . map (| shorts | { shorts . into_iter () . map (| s | { populate_arg_candidate (CompletionCandidate :: new (s . to_string ()) , a) . help (a . get_help () . cloned () . or_else (| | a . get_long () . map (| long | format ! ("--{long}") . into ())) ,) }) }) }) . flatten () . collect () }
};
}
