// Generated macro for impl_640 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_640 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_640"}
// Dependencies: {}
impl BufferElem { pub fn len (& self) -> u32 { match self { BufferElem :: Sized (sized) => mem :: size_of_val (sized . as_ref ()) , BufferElem :: Vector (vec) => vec . len () , } . try_into () . unwrap () } pub fn capacity (& self) -> u32 { match self { BufferElem :: Sized (sized) => mem :: size_of_val (sized . as_ref ()) , BufferElem :: Vector (vec) => vec . capacity () , } . try_into () . unwrap () } pub fn as_ptr (& self) -> * const u8 { match self { BufferElem :: Sized (sized) => ptr :: from_ref (sized . as_ref ()) . cast :: < u8 > () , BufferElem :: Vector (vec) => vec . as_ptr () , } } }
};
}
