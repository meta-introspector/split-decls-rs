// Generated macro for impl_2075 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2075 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2075"}
// Dependencies: {}
impl CSR { pub (crate) fn bits (self) -> Imm12 { Imm12 :: from_i16 (match self { CSR :: Frm => 0x0002 , }) } pub (crate) fn name (self) -> & 'static str { match self { CSR :: Frm => "frm" , } } }
};
}
