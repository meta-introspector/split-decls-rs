// Generated macro for impl_25 (impl)
macro_rules! Depcrate_codingimpl_25 {
() => {
// Module: crate::coding
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : Buf > BufExt for T { fn get < U : Codec > (& mut self) -> Result < U > { U :: decode (self) } fn get_var (& mut self) -> Result < u64 > { Ok (VarInt :: decode (self) ? . into_inner ()) } }
};
}
