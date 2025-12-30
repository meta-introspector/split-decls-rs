// Generated macro for impl_166 (impl)
macro_rules! Depcrate_dataimpl_166 {
() => {
// Module: crate::data
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: io :: Write for & NSMutableData { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } fn write_all (& mut self , buf : & [u8]) -> std :: io :: Result < () > { self . extend_from_slice (buf) ; Ok (()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
