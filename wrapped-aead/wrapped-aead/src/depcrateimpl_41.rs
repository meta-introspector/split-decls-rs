// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "arrayvec")] impl < const N : usize > Buffer for arrayvec :: ArrayVec < u8 , N > { fn extend_from_slice (& mut self , other : & [u8]) -> Result < () > { arrayvec :: ArrayVec :: try_extend_from_slice (self , other) . map_err (| _ | Error) } fn truncate (& mut self , len : usize) { arrayvec :: ArrayVec :: truncate (self , len) ; } }
};
}
