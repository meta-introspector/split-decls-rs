// Generated macro for impl_2139 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2139 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2139"}
// Dependencies: {}
impl fmt :: Display for VecAluOpRRRR { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut s = format ! ("{self:?}") ; s . make_ascii_lowercase () ; let (opcode , category) = s . split_at (s . len () - 2) ; f . write_str (& format ! ("{opcode}.{category}")) } }
};
}
