// Generated macro for CRC_16_SPI_FUJITSU (const)
macro_rules! Depcrate_algorithmCRC_16_SPI_FUJITSU {
() => {
// Module: crate::algorithm
// Provides: {"CRC_16_SPI_FUJITSU"}
// Dependencies: {}
# [doc = " # [`CRC-16/SPI-FUJITSU`][1]"] # [doc = ""] # [doc = " - `width`: `16` bits"] # [doc = " - `poly`: `0x1021` (reversed: `0x8408`)"] # [doc = " - `init`: `0x1d0f`"] # [doc = " - `refin`: `false`"] # [doc = " - `refout`: `false`"] # [doc = " - `xorout`: `0x0`"] # [doc = " - `check`: `0xe5cc`"] # [doc = " - `residue`: `0x0`"] # [doc = ""] # [doc = " [1]: https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-16-spi-fujitsu"] pub const CRC_16_SPI_FUJITSU : Algorithm < u16 > = Algorithm { width : 16 , poly : 0x1021 , init : 0x1d0f , refin : false , refout : false , xorout : 0x0 , check : 0xe5cc , residue : 0x0 } ;
};
}
