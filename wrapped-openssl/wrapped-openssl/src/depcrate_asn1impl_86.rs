// Generated macro for impl_86 (impl)
macro_rules! Depcrate_asn1impl_86 {
() => {
// Module: crate::asn1
// Provides: {"impl_86"}
// Dependencies: {}
impl Asn1TimeRef { # [doc = " Find difference between two times"] # [corresponds (ASN1_TIME_diff)] pub fn diff (& self , compare : & Self) -> Result < TimeDiff , ErrorStack > { let mut days = 0 ; let mut secs = 0 ; let other = compare . as_ptr () ; let err = unsafe { ffi :: ASN1_TIME_diff (& mut days , & mut secs , self . as_ptr () , other) } ; match err { 0 => Err (ErrorStack :: get ()) , _ => Ok (TimeDiff { days , secs }) , } } # [doc = " Compare two times"] # [corresponds (ASN1_TIME_compare)] pub fn compare (& self , other : & Self) -> Result < Ordering , ErrorStack > { let d = self . diff (other) ? ; if d . days > 0 || d . secs > 0 { return Ok (Ordering :: Less) ; } if d . days < 0 || d . secs < 0 { return Ok (Ordering :: Greater) ; } Ok (Ordering :: Equal) } }
};
}
