// Generated macro for TermSearchConfig (struct)
macro_rules! Depcrate_term_searchTermSearchConfig {
() => {
// Module: crate::term_search
// Provides: {"TermSearchConfig"}
// Dependencies: {}
# [doc = " Configuration options for the term search"] # [derive (Debug , Clone , Copy)] pub struct TermSearchConfig { # [doc = " Enable borrow checking, this guarantees the outputs of the `term_search` to borrow-check"] pub enable_borrowcheck : bool , # [doc = " Indicate when to squash multiple trees to `Many` as there are too many to keep track"] pub many_alternatives_threshold : usize , # [doc = " Fuel for term search in \"units of work\""] pub fuel : u64 , }
};
}
