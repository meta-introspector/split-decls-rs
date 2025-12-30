// Generated macro for AnsiX923 (struct)
macro_rules! DepcrateAnsiX923 {
() => {
// Module: crate
// Provides: {"AnsiX923"}
// Dependencies: {}
# [doc = " Pad block with zeros except the last byte which will be set to the number"] # [doc = " bytes."] # [doc = ""] # [doc = " ```"] # [doc = " use block_padding::{AnsiX923, Padding};"] # [doc = " use block_padding::array::{Array, typenum::U8};"] # [doc = ""] # [doc = " let msg = b\"test\";"] # [doc = " let pos = msg.len();"] # [doc = " let mut block: Array::<u8, U8> = [0xff; 8].into();"] # [doc = " block[..pos].copy_from_slice(msg);"] # [doc = " AnsiX923::pad(&mut block, pos);"] # [doc = " assert_eq!(&block[..], b\"test\\x00\\x00\\x00\\x04\");"] # [doc = " let res = AnsiX923::unpad(&block).unwrap();"] # [doc = " assert_eq!(res, msg);"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct AnsiX923 ;
};
}
