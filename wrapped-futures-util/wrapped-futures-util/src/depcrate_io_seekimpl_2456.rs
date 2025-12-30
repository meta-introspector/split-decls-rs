// Generated macro for impl_2456 (impl)
macro_rules! Depcrate_io_seekimpl_2456 {
() => {
// Module: crate::io::seek
// Provides: {"impl_2456"}
// Dependencies: {}
impl < 'a , S : AsyncSeek + ? Sized + Unpin > Seek < 'a , S > { pub (super) fn new (seek : & 'a mut S , pos : SeekFrom) -> Self { Self { seek , pos } } }
};
}
