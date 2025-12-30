// Generated macro for impl_267 (impl)
macro_rules! Depcrate_read_cfiimpl_267 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_267"}
// Dependencies: {}
impl < T , S > PartialEq for RegisterRuleMap < T , S > where T : ReaderOffset + PartialEq , S : UnwindContextStorage < T > , { fn eq (& self , rhs : & Self) -> bool { for & (reg , ref rule) in & * self . rules { debug_assert ! (rule . is_defined ()) ; if * rule != rhs . get (reg) { return false ; } } for & (reg , ref rhs_rule) in & * rhs . rules { debug_assert ! (rhs_rule . is_defined ()) ; if * rhs_rule != self . get (reg) { return false ; } } true } }
};
}
