// Generated macro for impl_2145 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2145 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2145"}
// Dependencies: {}
impl fmt :: Display for VecAluOpRRR { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let suffix_length = match self { VecAluOpRRR :: VmergeVVM | VecAluOpRRR :: VmergeVXM | VecAluOpRRR :: VfmergeVFM => 3 , _ => 2 , } ; let mut s = format ! ("{self:?}") ; s . make_ascii_lowercase () ; let (opcode , category) = s . split_at (s . len () - suffix_length) ; f . write_str (& format ! ("{opcode}.{category}")) } }
};
}
