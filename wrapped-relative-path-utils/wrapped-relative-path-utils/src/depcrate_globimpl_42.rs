// Generated macro for impl_42 (impl)
macro_rules! Depcrate_globimpl_42 {
() => {
// Module: crate::glob
// Provides: {"impl_42"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . kind { ErrorKind :: ReadDir (..) => write ! (f , "Error reading directory") , ErrorKind :: DirEntry (..) => write ! (f , "Error getting directory entry") , } } }
};
}
