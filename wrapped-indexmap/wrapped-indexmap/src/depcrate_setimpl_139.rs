// Generated macro for impl_139 (impl)
macro_rules! Depcrate_setimpl_139 {
() => {
// Module: crate::set
// Provides: {"impl_139"}
// Dependencies: {}
impl < T , S > Default for IndexSet < T , S > where S : Default , { # [doc = " Return an empty [`IndexSet`]"] fn default () -> Self { IndexSet { map : IndexMap :: default () , } } }
};
}
