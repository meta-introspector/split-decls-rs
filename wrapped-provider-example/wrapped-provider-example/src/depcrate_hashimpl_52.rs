// Generated macro for impl_52 (impl)
macro_rules! Depcrate_hashimpl_52 {
() => {
// Module: crate::hash
// Provides: {"impl_52"}
// Dependencies: {}
impl hash :: Context for Sha256Context { fn fork_finish (& self) -> hash :: Output { hash :: Output :: new (& self . 0 . clone () . finalize () [..]) } fn fork (& self) -> Box < dyn hash :: Context > { Box :: new (Self (self . 0 . clone ())) } fn finish (self : Box < Self >) -> hash :: Output { hash :: Output :: new (& self . 0 . finalize () [..]) } fn update (& mut self , data : & [u8]) { self . 0 . update (data) ; } }
};
}
