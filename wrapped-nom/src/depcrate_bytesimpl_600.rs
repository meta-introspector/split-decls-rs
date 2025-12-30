// Generated macro for impl_600 (impl)
macro_rules! Depcrate_bytesimpl_600 {
() => {
// Module: crate::bytes
// Provides: {"impl_600"}
// Dependencies: {}
impl < I , Error : ParseError < I > , F > Parser < I > for SplitPosition1 < F , Error > where I : Input , F : Fn (< I as Input > :: Item) -> bool , { type Output = I ; type Error = Error ; # [inline (always)] fn process < OM : OutputMode > (& mut self , i : I) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { i . split_at_position_mode1 :: < OM , _ , _ > (| c | (self . predicate) (c) , self . e) } }
};
}
