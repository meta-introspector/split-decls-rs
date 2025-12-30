// Generated macro for impl_327 (impl)
macro_rules! Depcrate_optionsimpl_327 {
() => {
// Module: crate::options
// Provides: {"impl_327"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : io :: Write > fmt :: Write for Adapter < T > { fn write_str (& mut self , s : & str) -> fmt :: Result { match self . writer . write_all (s . as_bytes ()) { Ok (()) => Ok (()) , Err (e) => { self . error = Err (e) ; Err (fmt :: Error) } } } }
};
}
