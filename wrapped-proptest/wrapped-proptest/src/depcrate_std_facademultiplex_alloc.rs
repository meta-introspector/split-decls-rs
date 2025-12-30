// Generated macro for multiplex_alloc (macro)
macro_rules! Depcrate_std_facademultiplex_alloc {
() => {
// Module: crate::std_facade
// Provides: {"multiplex_alloc"}
// Dependencies: {}
macro_rules ! multiplex_alloc { ($ ($ alloc : path , $ std : path) ,*) => { $ (# [cfg (all (feature = "alloc" , not (feature = "std")))] pub use $ alloc ; # [cfg (feature = "std")] pub use $ std ;) * } ; }
};
}
