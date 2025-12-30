// Generated macro for impl_319 (impl)
macro_rules! Depcrate_extensions_unicodeimpl_319 {
() => {
// Module: crate::extensions::unicode
// Provides: {"impl_319"}
// Dependencies: {}
impl writeable :: Writeable for Unicode { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { sink . write_char (UNICODE_EXT_CHAR) ? ; if ! self . attributes . is_empty () { sink . write_char ('-') ? ; writeable :: Writeable :: write_to (& self . attributes , sink) ? ; } if ! self . keywords . is_empty () { sink . write_char ('-') ? ; writeable :: Writeable :: write_to (& self . keywords , sink) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { if self . is_empty () { return writeable :: LengthHint :: exact (0) ; } let mut result = writeable :: LengthHint :: exact (1) ; if ! self . attributes . is_empty () { result += writeable :: Writeable :: writeable_length_hint (& self . attributes) + 1 ; } if ! self . keywords . is_empty () { result += writeable :: Writeable :: writeable_length_hint (& self . keywords) + 1 ; } result } }
};
}
