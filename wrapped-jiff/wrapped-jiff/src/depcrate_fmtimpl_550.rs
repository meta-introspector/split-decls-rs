// Generated macro for impl_550 (impl)
macro_rules! Depcrate_fmtimpl_550 {
() => {
// Module: crate::fmt
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'i , V > Parsed < 'i , V > { # [inline] fn and_then < U > (self , map : impl FnOnce (V) -> Result < U , Error > ,) -> Result < Parsed < 'i , U > , Error > { let Parsed { value , input } = self ; Ok (Parsed { value : map (value) ? , input }) } }
};
}
