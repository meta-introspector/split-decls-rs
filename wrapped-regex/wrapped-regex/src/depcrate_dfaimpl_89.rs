// Generated macro for impl_89 (impl)
macro_rules! Depcrate_dfaimpl_89 {
() => {
// Module: crate::dfa
// Provides: {"impl_89"}
// Dependencies: {}
impl State { fn flags (& self) -> StateFlags { StateFlags (self . data [0]) } fn inst_ptrs (& self) -> InstPtrs { InstPtrs { base : 0 , data : & self . data [1 ..] , } } }
};
}
