// Generated macro for impl_155 (impl)
macro_rules! Depcrate_check_consts_resolverimpl_155 {
() => {
// Module: crate::check_consts::resolver
// Provides: {"impl_155"}
// Dependencies: {}
impl JoinSemiLattice for State { fn join (& mut self , other : & Self) -> bool { self . qualif . join (& other . qualif) || self . borrow . join (& other . borrow) } }
};
}
