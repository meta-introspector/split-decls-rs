// Generated macro for Iso7816 (struct)
macro_rules! DepcrateIso7816 {
() => {
// Module: crate
// Provides: {"Iso7816"}
// Dependencies: {}
# [doc = " Pad block with byte sequence `\\x80 00...00 00`."] # [doc = ""] # [doc = " ```"] # [doc = " use block_padding::{Iso7816, Padding};"] # [doc = " use block_padding::array::{Array, typenum::U8};"] # [doc = ""] # [doc = " let msg = b\"test\";"] # [doc = " let pos = msg.len();"] # [doc = " let mut block: Array::<u8, U8> = [0xff; 8].into();"] # [doc = " block[..pos].copy_from_slice(msg);"] # [doc = " Iso7816::pad(&mut block, pos);"] # [doc = " assert_eq!(&block[..], b\"test\\x80\\x00\\x00\\x00\");"] # [doc = " let res = Iso7816::unpad(&block).unwrap();"] # [doc = " assert_eq!(res, msg);"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct Iso7816 ;
};
}
