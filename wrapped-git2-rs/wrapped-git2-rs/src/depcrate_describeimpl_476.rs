// Generated macro for impl_476 (impl)
macro_rules! Depcrate_describeimpl_476 {
() => {
// Module: crate::describe
// Provides: {"impl_476"}
// Dependencies: {}
impl < 'repo > Describe < 'repo > { # [doc = " Prints this describe result, returning the result as a string."] pub fn format (& self , opts : Option < & DescribeFormatOptions >) -> Result < String , Error > { let buf = Buf :: new () ; let opts = opts . map (| o | & o . raw as * const _) . unwrap_or (ptr :: null ()) ; unsafe { try_call ! (raw :: git_describe_format (buf . raw () , self . raw , opts)) ; } Ok (String :: from_utf8 (buf . to_vec ()) . unwrap ()) } }
};
}
