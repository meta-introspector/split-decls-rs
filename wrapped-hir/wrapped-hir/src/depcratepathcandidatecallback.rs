// Generated macro for PathCandidateCallback (trait)
macro_rules! DepcratePathCandidateCallback {
() => {
// Module: crate
// Provides: {"PathCandidateCallback"}
// Dependencies: {}
pub trait PathCandidateCallback { fn on_inherent_item (& mut self , item : AssocItem) -> ControlFlow < () > ; fn on_trait_item (& mut self , item : AssocItem) -> ControlFlow < () > ; }
};
}
