// Generated macro for impl_log_for_signed (macro)
macro_rules! Depcrate_loggerimpl_log_for_signed {
() => {
// Module: crate::logger
// Provides: {"impl_log_for_signed"}
// Dependencies: {}
# [doc = " Implement the log trait for the signed integer types."] macro_rules ! impl_log_for_signed { ($ type : tt) => { unsafe impl Log for $ type { # [inline] fn write_with_args (& self , buffer : & mut [MaybeUninit < u8 >] , args : & [Argument]) -> usize { if buffer . is_empty () { return 0 ; } match * self { 0 => { unsafe { buffer . get_unchecked_mut (0) . write (b'0') ; } 1 } value => { let mut prefix = 0 ; if * self < 0 { if buffer . len () == 1 { unsafe { buffer . get_unchecked_mut (0) . write (TRUNCATED) ; } return 1 ; } unsafe { buffer . get_unchecked_mut (0) . write (b'-') ; } prefix += 1 ; } ; prefix + $ type :: unsigned_abs (value) . write_with_args (& mut buffer [prefix ..] , args) } } } } } ; }
};
}
