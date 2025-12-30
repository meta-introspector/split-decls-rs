// Generated macro for multiplex_core (macro)
macro_rules! Depcrate_std_facademultiplex_core {
() => {
// Module: crate::std_facade
// Provides: {"multiplex_core"}
// Dependencies: {}
macro_rules ! multiplex_core { ($ ($ core : path , $ std : path) ,*) => { $ (# [cfg (not (feature = "std"))] pub use $ core ; # [cfg (feature = "std")] pub use $ std ;) * } ; }
};
}
