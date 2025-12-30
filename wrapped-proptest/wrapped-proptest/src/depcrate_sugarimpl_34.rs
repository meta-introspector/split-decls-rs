// Generated macro for impl_34 (impl)
macro_rules! Depcrate_sugarimpl_34 {
() => {
// Module: crate::sugar
// Provides: {"impl_34"}
// Dependencies: {}
impl < V : fmt :: Debug > fmt :: Debug for NamedArguments < & 'static str , V > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{} = " , self . 0) ? ; self . 1 . fmt (f) } }
};
}
