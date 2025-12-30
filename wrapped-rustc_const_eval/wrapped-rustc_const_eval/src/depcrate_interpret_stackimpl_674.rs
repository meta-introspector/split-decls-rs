// Generated macro for impl_674 (impl)
macro_rules! Depcrate_interpret_stackimpl_674 {
() => {
// Module: crate::interpret::stack
// Provides: {"impl_674"}
// Dependencies: {}
impl < 'tcx > fmt :: Display for FrameInfo < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ty :: tls :: with (| tcx | { if tcx . def_key (self . instance . def_id ()) . disambiguated_data . data == DefPathData :: Closure { write ! (f , "inside closure") } else { write ! (f , "inside `{}`" , self . instance) } }) } }
};
}
