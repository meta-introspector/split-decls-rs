// Generated macro for ServerAccess (enum)
macro_rules! Depcrate_generateServerAccess {
() => {
// Module: crate::generate
// Provides: {"ServerAccess"}
// Dependencies: {}
# [doc = " Server access code generation option"] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum ServerAccess { # [doc = " Supply a closure from ref to ref"] RefClosure , # [doc = " Supply a closure from ref to owned object which asrefs"] AsRefClosure , # [doc = " The interface is implemented for MethodInfo"] MethodInfo }
};
}
