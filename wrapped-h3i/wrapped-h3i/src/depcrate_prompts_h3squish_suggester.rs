// Generated macro for squish_suggester (function)
macro_rules! Depcrate_prompts_h3squish_suggester {
() => {
// Module: crate::prompts::h3
// Provides: {"squish_suggester"}
// Dependencies: {}
fn squish_suggester (suggestions : & [& str] , val : & str ,) -> SuggestionResult < Vec < String > > { let val_lower = val . to_lowercase () ; Ok (suggestions . iter () . filter (| s | s . to_lowercase () . contains (& val_lower)) . map (| s | String :: from (* s)) . collect ()) }
};
}
