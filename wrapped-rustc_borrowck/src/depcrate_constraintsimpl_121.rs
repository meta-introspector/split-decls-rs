// Generated macro for impl_121 (impl)
macro_rules! Depcrate_constraintsimpl_121 {
() => {
// Module: crate::constraints
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'tcx > fmt :: Debug for OutlivesConstraint < 'tcx > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "({:?}: {:?}) due to {:?} ({:?}) ({:?})" , self . sup , self . sub , self . locations , self . variance_info , self . category ,) } }
};
}
