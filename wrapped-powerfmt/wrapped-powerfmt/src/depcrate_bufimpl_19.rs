// Generated macro for impl_19 (impl)
macro_rules! Depcrate_bufimpl_19 {
() => {
// Module: crate::buf
// Provides: {"impl_19"}
// Dependencies: {}
impl < const SIZE : usize > core :: ops :: Deref for WriteBuffer < SIZE > { type Target = str ; fn deref (& self) -> & Self :: Target { unsafe { let s = maybe_uninit_slice_assume_init_ref (& self . buf [.. self . len]) ; str :: from_utf8_unchecked (s) } } }
};
}
