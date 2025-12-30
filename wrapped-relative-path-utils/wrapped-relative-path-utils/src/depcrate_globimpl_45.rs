// Generated macro for impl_45 (impl)
macro_rules! Depcrate_globimpl_45 {
() => {
// Module: crate::glob
// Provides: {"impl_45"}
// Dependencies: {}
impl std :: error :: Error for Error { # [allow (clippy :: match_same_arms)] fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match & self . kind { ErrorKind :: ReadDir (error) => Some (error) , ErrorKind :: DirEntry (error) => Some (error) , } } }
};
}
