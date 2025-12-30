// Generated macro for impl_15 (impl)
macro_rules! Depcrate_permissionimpl_15 {
() => {
// Module: crate::permission
// Provides: {"impl_15"}
// Dependencies: {}
impl < R > Display for Error < R > where R : std :: fmt :: Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Not allowed to handle resource {:?}: permission denied" , self . resource) } }
};
}
