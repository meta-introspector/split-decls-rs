// Generated macro for Read (trait)
macro_rules! Depcrate_no_stdRead {
() => {
// Module: crate::no_std
// Provides: {"Read"}
// Dependencies: {}
# [doc = " `no_std` compatible `std::io::Read` trait"] # [doc = ""] # [doc = " Will get removed once there is a standard way in either `core` or `alloc`."] pub trait Read { # [doc = " Read some bytes from this source into the specified buffer."] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > ; # [doc = " Read the exact number of bytes required to fill the buffer."] fn read_exact (& mut self , buf : & mut [u8]) -> crate :: Result < () > { default_read_exact (self , buf) } }
};
}
