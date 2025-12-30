// Generated macro for Pkcs7 (struct)
macro_rules! DepcratePkcs7 {
() => {
// Module: crate
// Provides: {"Pkcs7"}
// Dependencies: {}
# [doc = " Pad block with bytes with value equal to the number of bytes added."] # [doc = ""] # [doc = " PKCS#7 described in the [RFC 5652](https://tools.ietf.org/html/rfc5652#section-6.3)."] # [doc = ""] # [doc = " ```"] # [doc = " use block_padding::{Pkcs7, Padding};"] # [doc = " use block_padding::array::{Array, typenum::U8};"] # [doc = ""] # [doc = " let msg = b\"test\";"] # [doc = " let pos = msg.len();"] # [doc = " let mut block: Array::<u8, U8> = [0xff; 8].into();"] # [doc = " block[..pos].copy_from_slice(msg);"] # [doc = " Pkcs7::pad(&mut block, pos);"] # [doc = " assert_eq!(&block[..], b\"test\\x04\\x04\\x04\\x04\");"] # [doc = " let res = Pkcs7::unpad(&block).unwrap();"] # [doc = " assert_eq!(res, msg);"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct Pkcs7 ;
};
}
