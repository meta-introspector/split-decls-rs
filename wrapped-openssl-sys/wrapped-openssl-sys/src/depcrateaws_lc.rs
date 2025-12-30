// Generated macro for aws_lc (module)
macro_rules! Depcrateaws_lc {
() => {
// Module: crate
// Provides: {"aws_lc"}
// Dependencies: {}
# [cfg (awslc)] # [path = "."] mod aws_lc { # [cfg (all (feature = "aws-lc" , not (feature = "aws-lc-fips")))] pub use aws_lc_sys :: * ; # [cfg (feature = "aws-lc-fips")] pub use aws_lc_fips_sys :: * ; # [cfg (not (any (feature = "aws-lc" , feature = "aws-lc-fips")))] include ! (concat ! (env ! ("OUT_DIR") , "/bindgen.rs")) ; use libc :: { c_char , c_long , c_void } ; pub fn init () { unsafe { CRYPTO_library_init () } } # [allow (non_snake_case , clippy :: not_unsafe_ptr_arg_deref)] pub fn BIO_get_mem_data (b : * mut BIO , pp : * mut * mut c_char) -> c_long { unsafe { BIO_ctrl (b , BIO_CTRL_INFO , 0 , pp . cast :: < c_void > ()) } } }
};
}
