// Generated macro for impl_598 (impl)
macro_rules! Depcrate_bytesimpl_598 {
() => {
// Module: crate::bytes
// Provides: {"impl_598"}
// Dependencies: {}
impl < I , Error : ParseError < I > , F > Parser < I > for SplitPosition < F , Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { type Output = I ; type Error = Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { i . split_at_position_mode :: < OM , _ , _ > (| c | (self . predicate) (c)) } }
};
}
