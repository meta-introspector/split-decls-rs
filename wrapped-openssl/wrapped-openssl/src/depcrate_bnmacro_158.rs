// Generated macro for macro_158 (macro)
macro_rules! Depcrate_bnmacro_158 {
() => {
// Module: crate::bn
// Provides: {"macro_158"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: BN_CTX ; fn drop = ffi :: BN_CTX_free ; # [doc = " Temporary storage for BigNums on the secure heap"] # [doc = ""] # [doc = " BigNum values are stored dynamically and therefore can be expensive"] # [doc = " to allocate.  BigNumContext and the OpenSSL [`BN_CTX`] structure are used"] # [doc = " internally when passing BigNum values between subroutines."] # [doc = ""] # [doc = " [`BN_CTX`]: https://docs.openssl.org/master/man3/BN_CTX_new/"] pub struct BigNumContext ; # [doc = " Reference to [`BigNumContext`]"] # [doc = ""] # [doc = " [`BigNumContext`]: struct.BigNumContext.html"] pub struct BigNumContextRef ; }
};
}
