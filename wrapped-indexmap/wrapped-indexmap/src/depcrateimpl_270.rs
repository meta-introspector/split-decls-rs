// Generated macro for impl_270 (impl)
macro_rules! Depcrateimpl_270 {
() => {
// Module: crate
// Provides: {"impl_270"}
// Dependencies: {}
impl < K , V > Clone for Bucket < K , V > where K : Clone , V : Clone , { fn clone (& self) -> Self { Bucket { hash : self . hash , key : self . key . clone () , value : self . value . clone () , } } fn clone_from (& mut self , other : & Self) { self . hash = other . hash ; self . key . clone_from (& other . key) ; self . value . clone_from (& other . value) ; } }
};
}
