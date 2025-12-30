// Generated macro for impl_220 (impl)
macro_rules! Depcrate_dfa_minimizeimpl_220 {
() => {
// Module: crate::dfa::minimize
// Provides: {"impl_220"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Minimizer < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Minimizer") . field ("dfa" , & self . dfa) . field ("in_transitions" , & self . in_transitions) . field ("partitions" , & self . partitions) . field ("waiting" , & self . waiting) . finish () } }
};
}
