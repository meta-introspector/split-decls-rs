// Generated macro for impl_2116 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2116 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2116"}
// Dependencies: {}
impl VecAvl { pub fn _static (size : u32) -> Self { VecAvl :: Static { size : UImm5 :: maybe_from_u8 (size as u8) . expect ("Invalid size for AVL") , } } pub fn is_static (& self) -> bool { match self { VecAvl :: Static { .. } => true , } } pub fn unwrap_static (& self) -> UImm5 { match self { VecAvl :: Static { size } => * size , } } }
};
}
