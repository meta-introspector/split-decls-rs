// Generated macro for impl_5154 (impl)
macro_rules! Depcrate_machoimpl_5154 {
() => {
// Module: crate::macho
// Provides: {"impl_5154"}
// Dependencies: {}
impl RelocationInfo { # [doc = " Combine the fields into a `Relocation`."] pub fn relocation < E : Endian > (self , endian : E) -> Relocation < E > { let r_word0 = U32 :: new (endian , self . r_address) ; let r_word1 = U32 :: new (endian , if endian . is_little_endian () { self . r_symbolnum & 0x00ff_ffff | u32 :: from (self . r_pcrel) << 24 | u32 :: from (self . r_length & 0x3) << 25 | u32 :: from (self . r_extern) << 27 | u32 :: from (self . r_type) << 28 } else { self . r_symbolnum >> 8 | u32 :: from (self . r_pcrel) << 7 | u32 :: from (self . r_length & 0x3) << 5 | u32 :: from (self . r_extern) << 4 | u32 :: from (self . r_type) & 0xf } ,) ; Relocation { r_word0 , r_word1 } } }
};
}
