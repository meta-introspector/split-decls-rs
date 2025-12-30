// Generated macro for impl_788 (impl)
macro_rules! Depcrate_tyimpl_788 {
() => {
// Module: crate::ty
// Provides: {"impl_788"}
// Dependencies: {}
impl core :: ops :: Add < u32 > for EnumValue { type Output = Self ; fn add (self , n : u32) -> Self :: Output { match self { Self :: Unsigned (x) => Self :: Unsigned (x + u128 :: from (n)) , Self :: Signed (x) => Self :: Signed (x + i128 :: from (n)) , } } }
};
}
