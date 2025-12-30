// Generated macro for impl_7 (impl)
macro_rules! Depcrate_arbitraryimpl_7 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_7"}
// Dependencies: {}
impl Arbitrary < '_ > for ByteSize { fn arbitrary (u : & mut Unstructured < '_ >) -> arbitrary :: Result < Self > { Ok (ByteSize (u64 :: arbitrary (u) ?)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { u64 :: size_hint (depth) } }
};
}
