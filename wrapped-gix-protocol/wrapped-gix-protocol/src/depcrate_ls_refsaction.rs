// Generated macro for Action (enum)
macro_rules! Depcrate_ls_refsAction {
() => {
// Module: crate::ls_refs
// Provides: {"Action"}
// Dependencies: {}
# [doc = " What to do after preparing ls-refs in [`ls_refs()`][crate::ls_refs()]."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub enum Action { # [doc = " Continue by sending a 'ls-refs' command."] Continue , # [doc = " Skip 'ls-refs' entirely."] # [doc = ""] # [doc = " This is useful if the `ref-in-want` capability is taken advantage of. When fetching, one must must then send"] # [doc = " `want-ref`s during the negotiation phase."] Skip , }
};
}
