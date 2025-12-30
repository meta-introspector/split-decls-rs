// Generated macro for impl_5158 (impl)
macro_rules! Depcrate_machoimpl_5158 {
() => {
// Module: crate::macho
// Provides: {"impl_5158"}
// Dependencies: {}
impl ScatteredRelocationInfo { # [doc = " Combine the fields into a `Relocation`."] pub fn relocation < E : Endian > (self , endian : E) -> Relocation < E > { let r_word0 = U32 :: new (endian , self . r_address & 0x00ff_ffff | u32 :: from (self . r_type & 0xf) << 24 | u32 :: from (self . r_length & 0x3) << 28 | u32 :: from (self . r_pcrel) << 30 | R_SCATTERED ,) ; let r_word1 = U32 :: new (endian , self . r_value) ; Relocation { r_word0 , r_word1 } } }
};
}
