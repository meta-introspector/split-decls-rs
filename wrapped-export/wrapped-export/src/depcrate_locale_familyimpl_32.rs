// Generated macro for impl_32 (impl)
macro_rules! Depcrate_locale_familyimpl_32 {
() => {
// Module: crate::locale_family
// Provides: {"impl_32"}
// Dependencies: {}
impl Writeable for DataLocaleFamily { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { if let Some (locale) = self . locale . as_ref () { self . annotations . write_to (sink) ? ; locale . write_to (sink) } else { sink . write_str ("full") } } fn writeable_length_hint (& self) -> writeable :: LengthHint { if let Some (locale) = self . locale . as_ref () { self . annotations . writeable_length_hint () + locale . writeable_length_hint () } else { writeable :: LengthHint :: exact (4) } } }
};
}
