// Generated macro for PointerOffset (struct)
macro_rules! Depcrate_streamPointerOffset {
() => {
// Module: crate::stream
// Provides: {"PointerOffset"}
// Dependencies: {}
# [doc = " Newtype around a pointer offset into a slice stream (`&[T]`/`&str`)."] pub struct PointerOffset < T : ? Sized > (pub usize , PhantomData < T >) ;
};
}
