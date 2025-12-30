// Generated macro for impl_214 (impl)
macro_rules! Depcrate_handlerimpl_214 {
() => {
// Module: crate::handler
// Provides: {"impl_214"}
// Dependencies: {}
impl < I > fmt :: Debug for Response < I > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut fmt = fmt . debug_struct ("Response") ; match self . item { ResponseTypeItem :: Result (_) => fmt . field ("item" , & "Result(_)" . to_string ()) , ResponseTypeItem :: Fut (_) => fmt . field ("item" , & "Fut(_)" . to_string ()) , } . finish () } }
};
}
