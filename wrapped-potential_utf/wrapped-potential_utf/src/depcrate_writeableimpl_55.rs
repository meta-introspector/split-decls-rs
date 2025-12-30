// Generated macro for impl_55 (impl)
macro_rules! Depcrate_writeableimpl_55 {
() => {
// Module: crate::writeable
// Provides: {"impl_55"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `writeable` Cargo feature"] impl TryWriteable for & '_ PotentialUtf8 { type Error = Utf8Error ; fn try_write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { let mut remaining = & self . 0 ; let mut r = Ok (()) ; loop { match core :: str :: from_utf8 (remaining) { Ok (valid) => { sink . write_str (valid) ? ; return Ok (r) ; } Err (e) => { let valid = unsafe { core :: str :: from_utf8_unchecked (remaining . get_unchecked (.. e . valid_up_to ())) } ; sink . write_str (valid) ? ; sink . with_part (Part :: ERROR , | s | s . write_char (char :: REPLACEMENT_CHARACTER)) ? ; if r . is_ok () { r = Err (e) ; } let Some (error_len) = e . error_len () else { return Ok (r) ; } ; remaining = unsafe { remaining . get_unchecked (e . valid_up_to () + error_len ..) } } } } } fn writeable_length_hint (& self) -> LengthHint { LengthHint :: between (self . 0 . len () , self . 0 . len () * 3) } }
};
}
