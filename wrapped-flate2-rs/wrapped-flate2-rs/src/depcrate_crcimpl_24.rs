// Generated macro for impl_24 (impl)
macro_rules! Depcrate_crcimpl_24 {
() => {
// Module: crate::crc
// Provides: {"impl_24"}
// Dependencies: {}
impl Crc { # [doc = " Create a new CRC."] pub fn new () -> Self { Self :: default () } # [doc = " Returns the current crc32 checksum."] pub fn sum (& self) -> u32 { self . hasher . clone () . finalize () } # [doc = " The number of bytes that have been used to calculate the CRC."] # [doc = " This value is only accurate if the amount is lower than 2<sup>32</sup>."] pub fn amount (& self) -> u32 { self . amt } # [doc = " Update the CRC with the bytes in `data`."] pub fn update (& mut self , data : & [u8]) { self . amt = self . amt . wrapping_add (data . len () as u32) ; self . hasher . update (data) ; } # [doc = " Reset the CRC."] pub fn reset (& mut self) { self . amt = 0 ; self . hasher . reset () ; } # [doc = " Combine the CRC with the CRC for the subsequent block of bytes."] pub fn combine (& mut self , additional_crc : & Crc) { self . amt = self . amt . wrapping_add (additional_crc . amt) ; self . hasher . combine (& additional_crc . hasher) ; } }
};
}
