// Generated macro for impl_219 (impl)
macro_rules! Depcrate_handlerimpl_219 {
() => {
// Module: crate::handler
// Provides: {"impl_219"}
// Dependencies: {}
impl < A , I > fmt :: Debug for ActorResponse < A , I > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut fmt = fmt . debug_struct ("ActorResponse") ; match self . item { ActorResponseTypeItem :: Result (_) => fmt . field ("item" , & "Result(_)" . to_string ()) , ActorResponseTypeItem :: Fut (_) => fmt . field ("item" , & "Fut(_)" . to_string ()) , } . finish () } }
};
}
