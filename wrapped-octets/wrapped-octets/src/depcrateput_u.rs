// Generated macro for put_u (macro)
macro_rules! Depcrateput_u {
() => {
// Module: crate
// Provides: {"put_u"}
// Dependencies: {}
macro_rules ! put_u { ($ b : expr , $ ty : ty , $ v : expr , $ len : expr) => { { let len = $ len ; if $ b . buf . len () < $ b . off + len { return Err (BufferTooShortError) ; } let v = $ v ; let dst = & mut $ b . buf [$ b . off .. ($ b . off + len)] ; static_assert ! ($ len <= mem :: size_of ::<$ ty > ()) ; unsafe { let src = &<$ ty >:: to_be (v) as * const $ ty as * const u8 ; let off = (mem :: size_of ::<$ ty > () - len) as isize ; ptr :: copy_nonoverlapping (src . offset (off) , dst . as_mut_ptr () , len) ; } $ b . off += $ len ; Ok (dst) } } ; }
};
}
