// Generated macro for impl_common (macro)
macro_rules! Depcrate_int_bigimpl_common {
() => {
// Module: crate::int::big
// Provides: {"impl_common"}
// Dependencies: {}
macro_rules ! impl_common { ($ ty : ty) => { impl ops :: BitOr for $ ty { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self :: Output { self . 0 [0] |= rhs . 0 [0] ; self . 0 [1] |= rhs . 0 [1] ; self . 0 [2] |= rhs . 0 [2] ; self . 0 [3] |= rhs . 0 [3] ; self } } impl ops :: Not for $ ty { type Output = Self ; fn not (self) -> Self :: Output { Self ([! self . 0 [0] , ! self . 0 [1] , ! self . 0 [2] , ! self . 0 [3]]) } } impl ops :: Shl < u32 > for $ ty { type Output = Self ; fn shl (self , rhs : u32) -> Self :: Output { unimplemented ! ("only used to meet trait bounds") } } } ; }
};
}
