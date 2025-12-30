// Generated macro for impl_178 (impl)
macro_rules! Depcrate_connect_connectionimpl_178 {
() => {
// Module: crate::connect::connection
// Provides: {"impl_178"}
// Dependencies: {}
impl < R , IO > Connection < R , IO > { # [doc = " Deconstructs into IO and request parts."] pub fn into_parts (self) -> (IO , R) { (self . io , self . req) } # [doc = " Replaces underlying IO, returning old IO and new `Connection`."] pub fn replace_io < IO2 > (self , io : IO2) -> (IO , Connection < R , IO2 >) { (self . io , Connection { io , req : self . req }) } # [doc = " Returns a shared reference to the underlying IO."] pub fn io_ref (& self) -> & IO { & self . io } # [doc = " Returns a mutable reference to the underlying IO."] pub fn io_mut (& mut self) -> & mut IO { & mut self . io } # [doc = " Returns a reference to the connection request."] pub fn request (& self) -> & R { & self . req } }
};
}
