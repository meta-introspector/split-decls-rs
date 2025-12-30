// Generated macro for impl_279 (impl)
macro_rules! Depcrate_vecimpl_279 {
() => {
// Module: crate::vec
// Provides: {"impl_279"}
// Dependencies: {}
# [doc = " Write is implemented for `Vec<u8>` by appending to the vector."] # [doc = " The vector will grow as needed."] # [cfg (feature = "std")] impl < A : Allocator > io :: Write for Vec < u8 , A > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline] fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { let len = bufs . iter () . map (| b | b . len ()) . sum () ; self . reserve (len) ; for buf in bufs { self . extend_from_slice (buf) ; } Ok (len) } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . extend_from_slice (buf) ; Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
