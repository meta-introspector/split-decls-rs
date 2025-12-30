// Generated macro for impl_157 (impl)
macro_rules! Depcrate_collections_vecimpl_157 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_157"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'bump > io :: Write for Vec < 'bump , u8 > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . extend_from_slice_copy (buf) ; Ok (buf . len ()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . extend_from_slice_copy (buf) ; Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
