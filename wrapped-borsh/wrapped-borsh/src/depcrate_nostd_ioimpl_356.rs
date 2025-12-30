// Generated macro for impl_356 (impl)
macro_rules! Depcrate_nostd_ioimpl_356 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_356"}
// Dependencies: {}
# [doc = " Write is implemented for `Vec<u8>` by appending to the vector."] # [doc = " The vector will grow as needed."] impl Write for alloc :: vec :: Vec < u8 > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () > { self . extend_from_slice (buf) ; Ok (()) } # [inline] fn flush (& mut self) -> Result < () > { Ok (()) } }
};
}
