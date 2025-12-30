// Generated macro for impl_1349 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1349 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1349"}
// Dependencies: {}
impl ExtMode { # [doc = " Calculate the `ExtMode` from passed bit lengths of the from/to types."] pub (crate) fn new (from_bits : u16 , to_bits : u16) -> Option < ExtMode > { match (from_bits , to_bits) { (1 , 8) | (1 , 16) | (1 , 32) | (8 , 16) | (8 , 32) => Some (ExtMode :: BL) , (1 , 64) | (8 , 64) => Some (ExtMode :: BQ) , (16 , 32) => Some (ExtMode :: WL) , (16 , 64) => Some (ExtMode :: WQ) , (32 , 64) => Some (ExtMode :: LQ) , _ => None , } } # [doc = " Return the source register size in bytes."] pub (crate) fn src_size (& self) -> u8 { match self { ExtMode :: BL | ExtMode :: BQ => 1 , ExtMode :: WL | ExtMode :: WQ => 2 , ExtMode :: LQ => 4 , } } # [doc = " Return the destination register size in bytes."] pub (crate) fn dst_size (& self) -> u8 { match self { ExtMode :: BL | ExtMode :: WL => 4 , ExtMode :: BQ | ExtMode :: WQ | ExtMode :: LQ => 8 , } } # [doc = " Source size, as an integer type."] pub (crate) fn src_type (& self) -> Type { match self { ExtMode :: BL | ExtMode :: BQ => I8 , ExtMode :: WL | ExtMode :: WQ => I16 , ExtMode :: LQ => I32 , } } }
};
}
