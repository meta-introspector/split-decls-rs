// Generated macro for impl_2063 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2063 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2063"}
// Dependencies: {}
impl StoreOP { pub (crate) fn op_name (self) -> & 'static str { match self { Self :: Sb => "sb" , Self :: Sh => "sh" , Self :: Sw => "sw" , Self :: Sd => "sd" , Self :: Fsh => "fsh" , Self :: Fsw => "fsw" , Self :: Fsd => "fsd" , } } pub (crate) fn from_type (ty : Type) -> Self { match ty { F16 => Self :: Fsh , F32 => Self :: Fsw , F64 => Self :: Fsd , I8 => Self :: Sb , I16 => Self :: Sh , I32 => Self :: Sw , I64 => Self :: Sd , _ => unreachable ! () , } } pub (crate) fn size (& self) -> i64 { match self { Self :: Sb => 1 , Self :: Sh | Self :: Fsh => 2 , Self :: Sw | Self :: Fsw => 4 , Self :: Sd | Self :: Fsd => 8 , } } pub (crate) fn op_code (self) -> u32 { match self { Self :: Sb | Self :: Sh | Self :: Sw | Self :: Sd => 0b0100011 , Self :: Fsh | Self :: Fsw | Self :: Fsd => 0b0100111 , } } pub (crate) fn funct3 (self) -> u32 { match self { Self :: Sb => 0b000 , Self :: Sh => 0b001 , Self :: Sw => 0b010 , Self :: Sd => 0b011 , Self :: Fsh => 0b001 , Self :: Fsw => 0b010 , Self :: Fsd => 0b011 , } } }
};
}
