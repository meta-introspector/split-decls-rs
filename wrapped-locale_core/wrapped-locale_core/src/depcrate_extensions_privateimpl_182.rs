// Generated macro for impl_182 (impl)
macro_rules! Depcrate_extensions_privateimpl_182 {
() => {
// Module: crate::extensions::private
// Provides: {"impl_182"}
// Dependencies: {}
impl writeable :: Writeable for Private { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { if self . is_empty () { return Ok (()) ; } sink . write_char (PRIVATE_EXT_CHAR) ? ; for key in self . iter () { sink . write_char ('-') ? ; writeable :: Writeable :: write_to (key , sink) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { if self . is_empty () { return writeable :: LengthHint :: exact (0) ; } let mut result = writeable :: LengthHint :: exact (1) ; for key in self . iter () { result += writeable :: Writeable :: writeable_length_hint (key) + 1 ; } result } }
};
}
