// Generated macro for impl_199 (impl)
macro_rules! Depcrate_lengthimpl_199 {
() => {
// Module: crate::length
// Provides: {"impl_199"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for Length { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { Ok (Self :: new (u . arbitrary () ?)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { u32 :: size_hint (depth) } }
};
}
