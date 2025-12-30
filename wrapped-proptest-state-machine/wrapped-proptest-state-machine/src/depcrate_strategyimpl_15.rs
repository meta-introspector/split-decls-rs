// Generated macro for impl_15 (impl)
macro_rules! Depcrate_strategyimpl_15 {
() => {
// Module: crate::strategy
// Provides: {"impl_15"}
// Dependencies: {}
impl < State , Transition , StateStrategy , TransitionStrategy > Debug for Sequential < State , Transition , StateStrategy , TransitionStrategy > { fn fmt (& self , f : & mut Formatter) -> Result { f . debug_struct ("Sequential") . field ("size" , & self . size) . finish () } }
};
}
