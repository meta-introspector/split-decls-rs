// Generated macro for impl_234 (impl)
macro_rules! Depcrate_extensions_transformimpl_234 {
() => {
// Module: crate::extensions::transform
// Provides: {"impl_234"}
// Dependencies: {}
impl writeable :: Writeable for Transform { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { if self . is_empty () { return Ok (()) ; } sink . write_char (TRANSFORM_EXT_CHAR) ? ; if let Some (lang) = & self . lang { sink . write_char ('-') ? ; lang . write_lowercased_to (sink) ? ; } if ! self . fields . is_empty () { sink . write_char ('-') ? ; writeable :: Writeable :: write_to (& self . fields , sink) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { if self . is_empty () { return writeable :: LengthHint :: exact (0) ; } let mut result = writeable :: LengthHint :: exact (1) ; if let Some (lang) = & self . lang { result += writeable :: Writeable :: writeable_length_hint (lang) + 1 ; } if ! self . fields . is_empty () { result += writeable :: Writeable :: writeable_length_hint (& self . fields) + 1 ; } result } }
};
}
