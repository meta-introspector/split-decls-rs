// Generated macro for impl_800 (impl)
macro_rules! Depcrate_oidimpl_800 {
() => {
// Module: crate::oid
// Provides: {"impl_800"}
// Dependencies: {}
impl fmt :: Display for Oid { # [doc = " Hex-encode this Oid into a formatter."] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut dst = [0u8 ; raw :: GIT_OID_HEXSZ + 1] ; unsafe { raw :: git_oid_tostr (dst . as_mut_ptr () as * mut libc :: c_char , dst . len () as libc :: size_t , & self . raw ,) ; } let s = & dst [.. dst . iter () . position (| & a | a == 0) . unwrap ()] ; str :: from_utf8 (s) . unwrap () . fmt (f) } }
};
}
