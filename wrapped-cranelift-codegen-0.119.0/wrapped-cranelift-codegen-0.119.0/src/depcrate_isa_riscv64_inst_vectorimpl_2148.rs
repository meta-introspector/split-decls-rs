// Generated macro for impl_2148 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2148 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2148"}
// Dependencies: {}
impl fmt :: Display for VecAluOpRRImm5 { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let suffix_length = match self { VecAluOpRRImm5 :: VmergeVIM => 3 , _ => 2 , } ; let mut s = format ! ("{self:?}") ; s . make_ascii_lowercase () ; let (opcode , category) = s . split_at (s . len () - suffix_length) ; f . write_str (& format ! ("{opcode}.{category}")) } }
};
}
