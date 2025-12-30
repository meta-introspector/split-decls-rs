// Generated macro for impl_2146 (impl)
macro_rules! Depcrate_io_buf_readerimpl_2146 {
() => {
// Module: crate::io::buf_reader
// Provides: {"impl_2146"}
// Dependencies: {}
impl < R > BufReader < R > { delegate_access_inner ! (inner , R , ()) ; # [doc = " Returns a reference to the internally buffered data."] # [doc = ""] # [doc = " Unlike `fill_buf`, this will not attempt to fill the buffer if it is empty."] pub fn buffer (& self) -> & [u8] { & self . buffer [self . pos .. self . cap] } # [doc = " Invalidates all data in the internal buffer."] # [inline] fn discard_buffer (self : Pin < & mut Self >) { let this = self . project () ; * this . pos = 0 ; * this . cap = 0 ; } }
};
}
