// Generated macro for impl_2062 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2062 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2062"}
// Dependencies: {}
impl LoadOP { pub (crate) fn op_name (self) -> & 'static str { match self { Self :: Lb => "lb" , Self :: Lh => "lh" , Self :: Lw => "lw" , Self :: Lbu => "lbu" , Self :: Lhu => "lhu" , Self :: Lwu => "lwu" , Self :: Ld => "ld" , Self :: Flh => "flh" , Self :: Flw => "flw" , Self :: Fld => "fld" , } } pub (crate) fn from_type (ty : Type) -> Self { match ty { F16 => Self :: Flh , F32 => Self :: Flw , F64 => Self :: Fld , I8 => Self :: Lb , I16 => Self :: Lh , I32 => Self :: Lw , I64 => Self :: Ld , _ => unreachable ! () , } } pub (crate) fn size (& self) -> i64 { match self { Self :: Lb | Self :: Lbu => 1 , Self :: Lh | Self :: Lhu | Self :: Flh => 2 , Self :: Lw | Self :: Lwu | Self :: Flw => 4 , Self :: Ld | Self :: Fld => 8 , } } pub (crate) fn op_code (self) -> u32 { match self { Self :: Lb | Self :: Lh | Self :: Lw | Self :: Lbu | Self :: Lhu | Self :: Lwu | Self :: Ld => { 0b0000011 } Self :: Flh | Self :: Flw | Self :: Fld => 0b0000111 , } } pub (crate) fn funct3 (self) -> u32 { match self { Self :: Lb => 0b000 , Self :: Lh => 0b001 , Self :: Lw => 0b010 , Self :: Lwu => 0b110 , Self :: Lbu => 0b100 , Self :: Lhu => 0b101 , Self :: Ld => 0b011 , Self :: Flh => 0b001 , Self :: Flw => 0b010 , Self :: Fld => 0b011 , } } }
};
}
