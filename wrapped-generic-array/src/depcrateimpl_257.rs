// Generated macro for impl_257 (impl)
macro_rules! Depcrateimpl_257 {
() => {
// Module: crate
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a , T , N : ArrayLength > TryFrom < & 'a [T] > for & 'a GenericArray < T , N > { type Error = LengthError ; # [inline (always)] fn try_from (slice : & 'a [T]) -> Result < Self , Self :: Error > { GenericArray :: try_from_slice (slice) } }
};
}
