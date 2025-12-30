// Generated macro for impl_27 (impl)
macro_rules! Depcrate_codingimpl_27 {
() => {
// Module: crate::coding
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : BufMut > BufMutExt for T { fn write < U : Codec > (& mut self , x : U) { x . encode (self) ; } fn write_var (& mut self , x : u64) { VarInt :: from_u64 (x) . unwrap () . encode (self) ; } }
};
}
