// Generated macro for impl_IntoRepr (macro)
macro_rules! Depcrate_repr_numimpl_IntoRepr {
() => {
// Module: crate::repr::num
// Provides: {"impl_IntoRepr"}
// Dependencies: {}
# [doc = " Defines the implementation of [`IntoRepr`] for integer types"] macro_rules ! impl_IntoRepr { ($ t : ident , $ conv_ty : ident) => { impl IntoRepr for $ t { fn into_repr (self) -> Result < Repr , ToCompactStringError > { let num_digits = NumChars :: num_chars (self) ; let mut repr = Repr :: with_capacity (num_digits) . unwrap_with_msg () ; # [allow (unused_comparisons)] let is_nonnegative = self >= 0 ; let mut n = if is_nonnegative { self as $ conv_ty } else { (! (self as $ conv_ty)) . wrapping_add (1) } ; let mut curr = num_digits as isize ; unsafe { repr . set_len (num_digits) } ; let buf_ptr = unsafe { repr . as_mut_buf () . as_mut_ptr () } ; let lut_ptr = DEC_DIGITS_LUT . as_ptr () ; unsafe { if mem :: size_of ::<$ t > () >= 2 { while n >= 10000 { let rem = (n % 10000) as isize ; n /= 10000 ; let d1 = (rem / 100) << 1 ; let d2 = (rem % 100) << 1 ; curr -= 4 ; ptr :: copy_nonoverlapping (lut_ptr . offset (d1) , buf_ptr . offset (curr) , 2) ; ptr :: copy_nonoverlapping (lut_ptr . offset (d2) , buf_ptr . offset (curr + 2) , 2 ,) ; } } let mut n = n as isize ; if n >= 100 { let d1 = (n % 100) << 1 ; n /= 100 ; curr -= 2 ; ptr :: copy_nonoverlapping (lut_ptr . offset (d1) , buf_ptr . offset (curr) , 2) ; } if n < 10 { curr -= 1 ; * buf_ptr . offset (curr) = (n as u8) + b'0' ; } else { let d1 = n << 1 ; curr -= 2 ; ptr :: copy_nonoverlapping (lut_ptr . offset (d1) , buf_ptr . offset (curr) , 2) ; } if ! is_nonnegative { curr -= 1 ; * buf_ptr . offset (curr) = b'-' ; } } debug_assert_eq ! (curr , 0) ; Ok (repr) } } } ; }
};
}
