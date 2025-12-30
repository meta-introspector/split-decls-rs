// Generated macro for impl_25 (impl)
macro_rules! Depcrate_impls_slice_refimpl_25 {
() => {
// Module: crate::impls::slice_ref
// Provides: {"impl_25"}
// Dependencies: {}
# [doc = " Read is implemented for `&[u8]` by copying from the slice."] # [doc = ""] # [doc = " Note that reading updates the slice to point to the yet unread part."] # [doc = " The slice will be empty when EOF is reached."] impl Read for & [u8] { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { let amt = core :: cmp :: min (buf . len () , self . len ()) ; let (a , b) = self . split_at (amt) ; if amt == 1 { buf [0] = a [0] ; } else { buf [.. amt] . copy_from_slice (a) ; } * self = b ; Ok (amt) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , crate :: ReadExactError < Self :: Error > > { if self . len () < buf . len () { return Err (crate :: ReadExactError :: UnexpectedEof) ; } self . read (buf) ? ; Ok (()) } }
};
}
