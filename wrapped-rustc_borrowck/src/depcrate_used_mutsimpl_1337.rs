// Generated macro for impl_1337 (impl)
macro_rules! Depcrate_used_mutsimpl_1337 {
() => {
// Module: crate::used_muts
// Provides: {"impl_1337"}
// Dependencies: {}
impl GatherUsedMutsVisitor < '_ , '_ , '_ , '_ > { fn remove_never_initialized_mut_locals (& mut self , into : Place < '_ >) { self . never_initialized_mut_locals . swap_remove (& into . local) ; } }
};
}
