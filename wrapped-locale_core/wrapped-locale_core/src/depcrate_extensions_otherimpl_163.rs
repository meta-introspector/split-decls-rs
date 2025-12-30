// Generated macro for impl_163 (impl)
macro_rules! Depcrate_extensions_otherimpl_163 {
() => {
// Module: crate::extensions::other
// Provides: {"impl_163"}
// Dependencies: {}
impl writeable :: Writeable for Other { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { if self . keys . is_empty () { return Ok (()) ; } sink . write_str (self . get_ext_str ()) ? ; for key in self . keys . iter () { sink . write_char ('-') ? ; writeable :: Writeable :: write_to (key , sink) ? ; } Ok (()) } fn writeable_length_hint (& self) -> writeable :: LengthHint { if self . keys . is_empty () { return writeable :: LengthHint :: exact (0) ; } ; let mut result = writeable :: LengthHint :: exact (1) ; for key in self . keys . iter () { result += writeable :: Writeable :: writeable_length_hint (key) + 1 ; } result } }
};
}
