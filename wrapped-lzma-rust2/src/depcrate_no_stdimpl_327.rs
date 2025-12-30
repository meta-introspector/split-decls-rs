// Generated macro for impl_327 (impl)
macro_rules! Depcrate_no_stdimpl_327 {
() => {
// Module: crate::no_std
// Provides: {"impl_327"}
// Dependencies: {}
impl Write for & mut [u8] { # [inline (always)] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { if buf . is_empty () { return Ok (0) ; } if self . is_empty () { return Err (Error :: WriteZero ("&mut [u8] is too small")) ; } let write_len = buf . len () . min (self . len ()) ; self [.. write_len] . copy_from_slice (& buf [.. write_len]) ; let remaining = core :: mem :: take (self) ; * self = & mut remaining [write_len ..] ; Ok (write_len) } # [inline (always)] fn flush (& mut self) -> crate :: Result < () > { Ok (()) } }
};
}
