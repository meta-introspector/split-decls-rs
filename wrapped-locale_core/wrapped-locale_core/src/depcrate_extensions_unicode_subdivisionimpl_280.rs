// Generated macro for impl_280 (impl)
macro_rules! Depcrate_extensions_unicode_subdivisionimpl_280 {
() => {
// Module: crate::extensions::unicode::subdivision
// Provides: {"impl_280"}
// Dependencies: {}
impl writeable :: Writeable for SubdivisionId { # [inline] fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { sink . write_str (self . region . to_tinystr () . to_ascii_lowercase () . as_str ()) ? ; sink . write_str (self . suffix . as_str ()) } # [inline] fn writeable_length_hint (& self) -> writeable :: LengthHint { self . region . writeable_length_hint () + self . suffix . writeable_length_hint () } }
};
}
