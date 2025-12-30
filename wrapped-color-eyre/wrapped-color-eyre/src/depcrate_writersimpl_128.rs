// Generated macro for impl_128 (impl)
macro_rules! Depcrate_writersimpl_128 {
() => {
// Module: crate::writers
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "issue-url")] impl < B , H > fmt :: Display for Footer < B , H > where B : Display , H : Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut inner_f = FooterWriter { inner : & mut * f , had_output : false , } ; write ! (& mut inner_f , "{}" , self . body) ? ; if inner_f . had_output { self . footer . fmt (f) ? ; } Ok (()) } }
};
}
