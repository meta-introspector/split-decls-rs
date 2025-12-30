// Generated macro for impl_262 (impl)
macro_rules! Depcrate_read_cfiimpl_262 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_262"}
// Dependencies: {}
impl < T , S > Debug for RegisterRuleMap < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RegisterRuleMap") . field ("rules" , & self . rules) . finish () } }
};
}
