// Generated macro for impl_Integer (macro)
macro_rules! Depcrateimpl_Integer {
() => {
// Module: crate
// Provides: {"impl_Integer"}
// Dependencies: {}
macro_rules ! impl_Integer { ($ t : ty [len = $ max_len : expr] as $ large_unsigned : ty) => { impl Integer for $ t { const MAX_STR_LEN : usize = $ max_len ; } impl private :: Sealed for $ t { type Buffer = [MaybeUninit < u8 >; $ max_len] ; # [allow (unused_comparisons)] # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] fn write (self , buf : & mut [MaybeUninit < u8 >; $ max_len]) -> & str { let is_nonnegative = self >= 0 ; let mut n = if is_nonnegative { self as $ large_unsigned } else { (! (self as $ large_unsigned)) . wrapping_add (1) } ; let mut curr = buf . len () ; let buf_ptr = buf . as_mut_ptr () as * mut u8 ; let lut_ptr = DEC_DIGITS_LUT . as_ptr () ; while n >= 10000 { let rem = n % 10000 ; n /= 10000 ; let d1 = ((rem / 100) << 1) as usize ; let d2 = ((rem % 100) << 1) as usize ; curr -= 4 ; unsafe { ptr :: copy_nonoverlapping (lut_ptr . add (d1) , buf_ptr . add (curr) , 2) ; ptr :: copy_nonoverlapping (lut_ptr . add (d2) , buf_ptr . add (curr + 2) , 2) ; } } if n >= 100 { let d1 = ((n % 100) << 1) as usize ; n /= 100 ; curr -= 2 ; unsafe { ptr :: copy_nonoverlapping (lut_ptr . add (d1) , buf_ptr . add (curr) , 2) ; } } if n < 10 { curr -= 1 ; unsafe { * buf_ptr . add (curr) = (n as u8) + b'0' ; } } else { let d1 = (n << 1) as usize ; curr -= 2 ; unsafe { ptr :: copy_nonoverlapping (lut_ptr . add (d1) , buf_ptr . add (curr) , 2) ; } } if ! is_nonnegative { curr -= 1 ; unsafe { * buf_ptr . add (curr) = b'-' ; } } let len = buf . len () - curr ; let bytes = unsafe { slice :: from_raw_parts (buf_ptr . add (curr) , len) } ; unsafe { str :: from_utf8_unchecked (bytes) } } } } ; }
};
}
