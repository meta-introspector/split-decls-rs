// Generated macro for impl_952 (impl)
macro_rules! Depcrate_ir_memflagsimpl_952 {
() => {
// Module: crate::ir::memflags
// Provides: {"impl_952"}
// Dependencies: {}
impl AliasRegion { const fn from_bits (bits : u8) -> Option < Self > { match bits { 0b00 => None , 0b01 => Some (Self :: Heap) , 0b10 => Some (Self :: Table) , 0b11 => Some (Self :: Vmctx) , _ => panic ! ("invalid alias region bits") , } } const fn to_bits (region : Option < Self >) -> u8 { match region { None => 0b00 , Some (r) => r as u8 , } } }
};
}
