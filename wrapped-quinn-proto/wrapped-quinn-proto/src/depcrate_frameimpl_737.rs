// Generated macro for impl_737 (impl)
macro_rules! Depcrate_frameimpl_737 {
() => {
// Module: crate::frame
// Provides: {"impl_737"}
// Dependencies: {}
impl EcnCounts { pub const ZERO : Self = Self { ect0 : 0 , ect1 : 0 , ce : 0 , } ; pub fn encode < W : BufMut > (& self , out : & mut W) { out . write_var (self . ect0) ; out . write_var (self . ect1) ; out . write_var (self . ce) ; } }
};
}
