// Generated macro for impl_100 (impl)
macro_rules! Depcrate_io_limitedimpl_100 {
() => {
// Module: crate::io::limited
// Provides: {"impl_100"}
// Dependencies: {}
impl < Io > Limited < Io > { pub (crate) fn new (io : Io , limit : usize) -> Self { Self { io , limit } } # [doc = " Acquires a reference to the underlying I/O object that this adaptor is"] # [doc = " wrapping."] pub fn get_ref (& self) -> & Io { & self . io } # [doc = " Acquires a mutable reference to the underlying I/O object that this"] # [doc = " adaptor is wrapping."] pub fn get_mut (& mut self) -> & mut Io { & mut self . io } # [doc = " Acquires a pinned mutable reference to the underlying I/O object that"] # [doc = " this adaptor is wrapping."] pub fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut Io > { self . project () . io } # [doc = " Consumes this adaptor returning the underlying I/O object."] pub fn into_inner (self) -> Io { self . io } }
};
}
