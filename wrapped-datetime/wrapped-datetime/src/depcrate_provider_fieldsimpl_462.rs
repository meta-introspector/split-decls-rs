// Generated macro for impl_462 (impl)
macro_rules! Depcrate_provider_fieldsimpl_462 {
() => {
// Module: crate::provider::fields
// Provides: {"impl_462"}
// Dependencies: {}
impl Writeable for Field { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { let ch : char = self . symbol . into () ; for _ in 0 .. self . length . to_len () { sink . write_char (ch) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { writeable :: LengthHint :: exact (self . length . to_len ()) } }
};
}
