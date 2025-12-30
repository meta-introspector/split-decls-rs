// Generated macro for impl_274 (impl)
macro_rules! Depcrate_varintimpl_274 {
() => {
// Module: crate::varint
// Provides: {"impl_274"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'arbitrary > Arbitrary < 'arbitrary > for VarInt { fn arbitrary (u : & mut arbitrary :: Unstructured < 'arbitrary >) -> arbitrary :: Result < Self > { Ok (Self (u . int_in_range (0 ..= Self :: MAX . 0) ?)) } }
};
}
