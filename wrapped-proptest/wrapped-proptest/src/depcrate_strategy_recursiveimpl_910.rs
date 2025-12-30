// Generated macro for impl_910 (impl)
macro_rules! Depcrate_strategy_recursiveimpl_910 {
() => {
// Module: crate::strategy::recursive
// Provides: {"impl_910"}
// Dependencies: {}
impl < T : fmt :: Debug , F > fmt :: Debug for Recursive < T , F > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Recursive") . field ("base" , & self . base) . field ("recurse" , & "<function>") . field ("depth" , & self . depth) . field ("desired_size" , & self . desired_size) . field ("expected_branch_size" , & self . expected_branch_size) . finish () } }
};
}
