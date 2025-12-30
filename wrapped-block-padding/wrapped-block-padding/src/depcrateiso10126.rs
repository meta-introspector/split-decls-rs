// Generated macro for Iso10126 (struct)
macro_rules! DepcrateIso10126 {
() => {
// Module: crate
// Provides: {"Iso10126"}
// Dependencies: {}
# [doc = " Pad block with arbitrary bytes ending with value equal to the number of bytes added."] # [doc = ""] # [doc = " A variation of PKCS#7 that is less strict when decoding."] # [doc = ""] # [doc = " ```"] # [doc = " use block_padding::{Iso10126, Padding};"] # [doc = " use block_padding::array::{Array, typenum::U8};"] # [doc = ""] # [doc = " let msg = b\"test\";"] # [doc = " let pos = msg.len();"] # [doc = " let mut block: Array::<u8, U8> = [0xff; 8].into();"] # [doc = " block[..pos].copy_from_slice(msg);"] # [doc = " Iso10126::pad(&mut block, pos);"] # [doc = " assert_eq!(&block[..], b\"test\\x04\\x04\\x04\\x04\");"] # [doc = " let res = Iso10126::unpad(&block).unwrap();"] # [doc = " assert_eq!(res, msg);"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct Iso10126 ;
};
}
