// Generated macro for impl_325 (impl)
macro_rules! Depcrate_rust_typeimpl_325 {
() => {
// Module: crate::rust_type
// Provides: {"impl_325"}
// Dependencies: {}
impl TypeSafety { const SAFE : Self = Self { in_argument : SafetyProperty :: Safe , in_return : SafetyProperty :: Safe , } ; fn always_unsafe (reason : impl Into < String > + Clone) -> Self { Self { in_argument : SafetyProperty :: new_unsafe (reason . clone ()) , in_return : SafetyProperty :: new_unsafe (reason) , } } fn unknown_in_argument (reason : impl Into < String >) -> Self { Self { in_argument : SafetyProperty :: new_unknown (reason) , in_return : SafetyProperty :: Safe , } } fn unsafe_in_argument (reason : impl Into < String >) -> Self { Self { in_argument : SafetyProperty :: new_unsafe (reason) , in_return : SafetyProperty :: Safe , } } fn unsafe_in_return (reason : impl Into < String >) -> Self { Self { in_argument : SafetyProperty :: Safe , in_return : SafetyProperty :: new_unsafe (reason) , } } fn merge (self , other : Self) -> Self { Self { in_argument : self . in_argument . merge (other . in_argument) , in_return : self . in_return . merge (other . in_return) , } } fn ignore_in_argument (self) -> Self { Self { in_argument : self . in_argument . ignore () , in_return : self . in_return , } } fn context (self , context : impl Display + Clone) -> Self { Self { in_argument : self . in_argument . context (context . clone ()) , in_return : self . in_return . context (context) , } } }
};
}
