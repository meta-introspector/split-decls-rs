// Generated macro for impl_20 (impl)
macro_rules! Depcrate_impls_slice_mutimpl_20 {
() => {
// Module: crate::impls::slice_mut
// Provides: {"impl_20"}
// Dependencies: {}
# [doc = " Write is implemented for `&mut [u8]` by copying into the slice, overwriting"] # [doc = " its data."] # [doc = ""] # [doc = " Note that writing updates the slice to point to the yet unwritten part."] # [doc = " The slice will be empty when it has been completely overwritten."] # [doc = ""] # [doc = " If the number of bytes to be written exceeds the size of the slice, write operations will"] # [doc = " return short writes: ultimately, a `SliceWriteError::Full`."] impl Write for & mut [u8] { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { let amt = core :: cmp :: min (buf . len () , self . len ()) ; if ! buf . is_empty () && amt == 0 { return Err (SliceWriteError :: Full) ; } let (a , b) = mem :: take (self) . split_at_mut (amt) ; a . copy_from_slice (& buf [.. amt]) ; * self = b ; Ok (amt) } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { if self . len () < buf . len () { return Err (SliceWriteError :: Full) ; } self . write (buf) ? ; Ok (()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
