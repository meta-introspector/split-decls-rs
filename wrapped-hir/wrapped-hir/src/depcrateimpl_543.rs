// Generated macro for impl_543 (impl)
macro_rules! Depcrateimpl_543 {
() => {
// Module: crate
// Provides: {"impl_543"}
// Dependencies: {}
impl < F > PathCandidateCallback for F where F : FnMut (AssocItem) -> ControlFlow < () > , { fn on_inherent_item (& mut self , item : AssocItem) -> ControlFlow < () > { self (item) } fn on_trait_item (& mut self , item : AssocItem) -> ControlFlow < () > { self (item) } }
};
}
