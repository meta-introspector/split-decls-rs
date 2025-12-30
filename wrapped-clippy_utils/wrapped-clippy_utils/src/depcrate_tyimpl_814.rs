// Generated macro for impl_814 (impl)
macro_rules! Depcrate_tyimpl_814 {
() => {
// Module: crate::ty
// Provides: {"impl_814"}
// Dependencies: {}
impl core :: ops :: Add < u32 > for EnumValue { type Output = Self ; fn add (self , n : u32) -> Self :: Output { match self { Self :: Unsigned (x) => Self :: Unsigned (x + u128 :: from (n)) , Self :: Signed (x) => Self :: Signed (x + i128 :: from (n)) , } } }
};
}
