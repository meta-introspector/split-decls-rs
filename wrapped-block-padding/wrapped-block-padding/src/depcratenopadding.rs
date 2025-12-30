// Generated macro for NoPadding (struct)
macro_rules! DepcrateNoPadding {
() => {
// Module: crate
// Provides: {"NoPadding"}
// Dependencies: {}
# [doc = " Don't pad the data. Useful for key wrapping."] # [doc = ""] # [doc = " ```"] # [doc = " use block_padding::{NoPadding, Padding};"] # [doc = " use block_padding::array::{Array, typenum::U8};"] # [doc = ""] # [doc = " let msg = b\"test\";"] # [doc = " let pos = msg.len();"] # [doc = " let mut block: Array::<u8, U8> = [0xff; 8].into();"] # [doc = " block[..pos].copy_from_slice(msg);"] # [doc = " NoPadding::pad(&mut block, pos);"] # [doc = " assert_eq!(&block[..], b\"test\\xff\\xff\\xff\\xff\");"] # [doc = " let res = NoPadding::unpad(&block).unwrap();"] # [doc = " assert_eq!(res, b\"test\\xff\\xff\\xff\\xff\");"] # [doc = " ```"] # [doc = ""] # [doc = " Note that even though the passed length of the message is equal to 4,"] # [doc = " the size of unpadded message is equal to the block size of 8 bytes."] # [doc = " Also padded message contains \"garbage\" bytes stored in the block buffer."] # [doc = " Thus `NoPadding` generally should not be used with data length of which"] # [doc = " is not multiple of block size."] # [derive (Clone , Copy , Debug)] pub struct NoPadding ;
};
}
