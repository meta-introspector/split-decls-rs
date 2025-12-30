// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl < const MAX_SIZE : usize > fmt :: Display for ObjectIdentifier < MAX_SIZE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . as_oid_ref ()) } }
};
}
