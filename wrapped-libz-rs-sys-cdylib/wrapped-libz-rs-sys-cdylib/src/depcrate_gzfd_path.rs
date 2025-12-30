// Generated macro for fd_path (function)
macro_rules! Depcrate_gzfd_path {
() => {
// Module: crate::gz
// Provides: {"fd_path"}
// Dependencies: {}
fn fd_path (buf : & mut [u8 ; 27] , fd : c_int) -> & CStr { use core :: fmt :: Write ; struct Writer < 'a > { buf : & 'a mut [u8 ; 27] , len : usize , } impl Write for Writer < '_ > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { let Some (dst) = self . buf . get_mut (self . len .. self . len + s . len ()) else { return Err (core :: fmt :: Error) ; } ; dst . copy_from_slice (s . as_bytes ()) ; self . len += s . len () ; Ok (()) } } let mut w = Writer { buf , len : 0 } ; write ! (w , "<fd:{fd}>\0") . unwrap () ; unsafe { CStr :: from_ptr (w . buf [.. w . len] . as_ptr () . cast ()) } }
};
}
