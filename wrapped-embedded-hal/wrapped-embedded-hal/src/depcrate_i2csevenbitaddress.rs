// Generated macro for SevenBitAddress (type)
macro_rules! Depcrate_i2cSevenBitAddress {
() => {
// Module: crate::i2c
// Provides: {"SevenBitAddress"}
// Dependencies: {}
# [doc = " 7-bit address mode type."] # [doc = ""] # [doc = " Note that 7-bit addresses defined by drivers should be specified in **right-aligned** form,"] # [doc = " e.g. in the range `0x00..=0x7F`."] # [doc = ""] # [doc = " For example, a device that has the seven bit address of `0b011_0010`, and therefore is addressed on the wire using:"] # [doc = ""] # [doc = " * `0b0110010_0` or `0x64` for *writes*"] # [doc = " * `0b0110010_1` or `0x65` for *reads*"] # [doc = ""] # [doc = " Should be specified as `0b0011_0010` or `0x32`, NOT `0x64` or `0x65`. Care should be taken by both HAL and driver"] # [doc = " crate writers to use this scheme consistently."] pub type SevenBitAddress = u8 ;
};
}
