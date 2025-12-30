// Generated macro for impl_130 (impl)
macro_rules! Depcrate_fmtimpl_130 {
() => {
// Module: crate::fmt
// Provides: {"impl_130"}
// Dependencies: {}
impl < W > std :: fmt :: Write for Adapter < W > where W : FnMut (& [u8]) -> std :: io :: Result < () > , { fn write_str (& mut self , s : & str) -> std :: fmt :: Result { match (self . writer) (s . as_bytes ()) { Ok (()) => Ok (()) , Err (e) => { self . error = Err (e) ; Err (std :: fmt :: Error) } } } }
};
}
