// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Buffer for Vec < u8 > { fn extend_from_slice (& mut self , other : & [u8]) -> Result < () > { Vec :: extend_from_slice (self , other) ; Ok (()) } fn truncate (& mut self , len : usize) { Vec :: truncate (self , len) ; } }
};
}
