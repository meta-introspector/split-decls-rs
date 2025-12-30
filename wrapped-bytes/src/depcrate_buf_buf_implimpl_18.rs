// Generated macro for impl_18 (impl)
macro_rules! Depcrate_buf_buf_implimpl_18 {
() => {
// Module: crate::buf::buf_impl
// Provides: {"impl_18"}
// Dependencies: {}
impl Buf for & [u8] { # [inline] fn remaining (& self) -> usize { self . len () } # [inline] fn chunk (& self) -> & [u8] { self } # [inline] fn advance (& mut self , cnt : usize) { if self . len () < cnt { panic_advance (& TryGetError { requested : cnt , available : self . len () , }) ; } * self = & self [cnt ..] ; } # [inline] fn copy_to_slice (& mut self , dst : & mut [u8]) { if self . len () < dst . len () { panic_advance (& TryGetError { requested : dst . len () , available : self . len () , }) ; } dst . copy_from_slice (& self [.. dst . len ()]) ; self . advance (dst . len ()) ; } }
};
}
