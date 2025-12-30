// Generated macro for impl_log_for_slice (macro)
macro_rules! Depcrate_loggerimpl_log_for_slice {
() => {
// Module: crate::logger
// Provides: {"impl_log_for_slice"}
// Dependencies: {}
# [doc = " Implement the log trait for the slice type."] macro_rules ! impl_log_for_slice { ([$ type : ident]) => { unsafe impl <$ type > Log for & [$ type] where $ type : Log { impl_log_for_slice ! (@ generate_write) ; } } ; ([$ type : ident ; $ size : ident]) => { unsafe impl <$ type , const $ size : usize > Log for & [$ type ; $ size] where $ type : Log { impl_log_for_slice ! (@ generate_write) ; } } ; (@ generate_write) => { # [inline] fn write_with_args (& self , buffer : & mut [MaybeUninit < u8 >] , _args : & [Argument]) -> usize { if buffer . is_empty () { return 0 ; } let length = buffer . len () ; unsafe { buffer . get_unchecked_mut (0) . write (b'[') ; } let mut offset = 1 ; for value in self . iter () { if offset >= length { unsafe { buffer . get_unchecked_mut (length - 1) . write (TRUNCATED) ; } offset = length ; break ; } if offset > 1 { if offset + 2 >= length { unsafe { buffer . get_unchecked_mut (length - 1) . write (TRUNCATED) ; } offset = length ; break ; } else { unsafe { buffer . get_unchecked_mut (offset) . write (b',') ; buffer . get_unchecked_mut (offset + 1) . write (b' ') ; } offset += 2 ; } } offset += value . debug (& mut buffer [offset ..]) ; } if offset < length { unsafe { buffer . get_unchecked_mut (offset) . write (b']') ; } offset += 1 ; } offset } } ; }
};
}
