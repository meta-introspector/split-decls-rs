// Generated macro for impl_344 (impl)
macro_rules! Depcrate_utilsimpl_344 {
() => {
// Module: crate::utils
// Provides: {"impl_344"}
// Dependencies: {}
impl SuperTraits < '_ > { fn elaborate (& mut self , trait_ref : & TraitRef) { direct_super_trait_refs (self . db , trait_ref , | trait_ref | { if ! self . seen . contains (& trait_ref . trait_id) { self . stack . push (trait_ref) ; } }) ; } }
};
}
