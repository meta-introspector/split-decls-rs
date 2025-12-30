// Generated macro for impl_1365 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1365 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1365"}
// Dependencies: {}
impl OperandSize { pub (crate) fn from_bytes (num_bytes : u32) -> Self { match num_bytes { 1 => OperandSize :: Size8 , 2 => OperandSize :: Size16 , 4 => OperandSize :: Size32 , 8 => OperandSize :: Size64 , _ => unreachable ! ("Invalid OperandSize: {}" , num_bytes) , } } pub (crate) fn from_ty (ty : Type) -> Self { Self :: from_bytes (ty . lane_type () . bytes ()) } pub (crate) fn is_one_of (& self , sizes : & [Self]) -> bool { sizes . iter () . any (| val | * self == * val) } pub (crate) fn to_bytes (& self) -> u8 { match self { Self :: Size8 => 1 , Self :: Size16 => 2 , Self :: Size32 => 4 , Self :: Size64 => 8 , } } pub (crate) fn to_bits (& self) -> u8 { self . to_bytes () * 8 } pub (crate) fn to_type (& self) -> Type { match self { Self :: Size8 => I8 , Self :: Size16 => I16 , Self :: Size32 => I32 , Self :: Size64 => I64 , } } }
};
}
