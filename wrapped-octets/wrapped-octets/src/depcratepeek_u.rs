// Generated macro for peek_u (macro)
macro_rules! Depcratepeek_u {
() => {
// Module: crate
// Provides: {"peek_u"}
// Dependencies: {}
macro_rules ! peek_u { ($ b : expr , $ ty : ty , $ len : expr) => { { let len = $ len ; let src = &$ b . buf [$ b . off ..] ; if src . len () < len { return Err (BufferTooShortError) ; } static_assert ! ($ len <= mem :: size_of ::<$ ty > ()) ; let mut out : $ ty = 0 ; unsafe { let dst = & mut out as * mut $ ty as * mut u8 ; let off = (mem :: size_of ::<$ ty > () - len) as isize ; ptr :: copy_nonoverlapping (src . as_ptr () , dst . offset (off) , len) ; } ; Ok (<$ ty >:: from_be (out)) } } ; }
};
}
