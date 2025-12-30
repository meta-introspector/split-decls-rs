// Generated macro for macro_160 (macro)
macro_rules! Depcrate_bnmacro_160 {
() => {
// Module: crate::bn
// Provides: {"macro_160"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: BIGNUM ; fn drop = ffi :: BN_free ; # [doc = " Dynamically sized large number implementation"] # [doc = ""] # [doc = " Perform large number mathematics.  Create a new BigNum"] # [doc = " with [`new`].  Perform standard mathematics on large numbers using"] # [doc = " methods from [`Dref<Target = BigNumRef>`]"] # [doc = ""] # [doc = " OpenSSL documentation at [`BN_new`]."] # [doc = ""] # [doc = " [`new`]: struct.BigNum.html#method.new"] # [doc = " [`Dref<Target = BigNumRef>`]: struct.BigNum.html#deref-methods"] # [doc = " [`BN_new`]: https://docs.openssl.org/master/man3/BN_new/"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use openssl::bn::BigNum;"] # [doc = " # use openssl::error::ErrorStack;"] # [doc = " # fn bignums() -> Result< (), ErrorStack > {"] # [doc = " let little_big = BigNum::from_u32(std::u32::MAX)?;"] # [doc = " assert_eq!(*&little_big.num_bytes(), 4);"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " # fn main () { bignums(); }"] # [doc = " ```"] pub struct BigNum ; # [doc = " Reference to a [`BigNum`]"] # [doc = ""] # [doc = " [`BigNum`]: struct.BigNum.html"] pub struct BigNumRef ; }
};
}
