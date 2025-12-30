// Generated macro for impl_61 (impl)
macro_rules! Depcrate_bytesimpl_61 {
() => {
// Module: crate::bytes
// Provides: {"impl_61"}
// Dependencies: {}
# [doc = " Implemented by hand because the derive would create invalid values."] # [doc = " Makes sure the length and the inner.len matches."] # [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for & 'a BytesRef { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let length : Length = u . arbitrary () ? ; Ok (BytesRef :: new_unchecked (u . bytes (u32 :: from (length) as usize) ? ,)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { arbitrary :: size_hint :: and (Length :: size_hint (depth) , (0 , None)) } }
};
}
