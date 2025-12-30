// Generated macro for data_ce_to_primary (function)
macro_rules! Depcrate_providerdata_ce_to_primary {
() => {
// Module: crate::provider
// Provides: {"data_ce_to_primary"}
// Dependencies: {}
fn data_ce_to_primary (data_ce : u64 , c : char) -> u32 { let p = (data_ce >> 32) as u32 ; let lower32 = data_ce as u32 as i32 ; let mut offset = ((u32 :: from (c) as i32) - (lower32 >> 8)) * (lower32 & 0x7F) ; let is_compressible = (lower32 & 0x80) != 0 ; offset += (((p >> 8) & 0xFF) as i32) - 2 ; let mut primary = (((offset % 254) + 2) as u32) << 8 ; offset /= 254 ; if is_compressible { offset += (((p >> 16) & 0xFF) as i32) - 4 ; primary |= (((offset % 251) + 4) as u32) << 16 ; offset /= 251 ; } else { offset += (((p >> 16) & 0xFF) as i32) - 2 ; primary |= (((offset % 254) + 2) as u32) << 16 ; offset /= 254 ; } primary | ((p & 0xFF000000) + ((offset as u32) << 24)) }
};
}
