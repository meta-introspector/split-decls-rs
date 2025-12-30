// Generated macro for ZeroPadding (struct)
macro_rules! DepcrateZeroPadding {
() => {
// Module: crate
// Provides: {"ZeroPadding"}
// Dependencies: {}
# [doc = " Pad block with zeros."] # [doc = ""] # [doc = " ```"] # [doc = " use block_padding::{ZeroPadding, Padding};"] # [doc = " use block_padding::array::{Array, typenum::U8};"] # [doc = ""] # [doc = " let msg = b\"test\";"] # [doc = " let pos = msg.len();"] # [doc = " let mut block: Array::<u8, U8> = [0xff; 8].into();"] # [doc = " block[..pos].copy_from_slice(msg);"] # [doc = " ZeroPadding::pad(&mut block, pos);"] # [doc = " assert_eq!(&block[..], b\"test\\x00\\x00\\x00\\x00\");"] # [doc = " let res = ZeroPadding::unpad(&mut block).unwrap();"] # [doc = " assert_eq!(res, msg);"] # [doc = " ```"] # [doc = ""] # [doc = " Note that zero padding is not reversible for messages which end"] # [doc = " with one or more zero bytes."] # [derive (Clone , Copy , Debug)] pub struct ZeroPadding ;
};
}
