// Generated macro for impl_258 (impl)
macro_rules! Depcrateimpl_258 {
() => {
// Module: crate
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a , T , N : ArrayLength > TryFrom < & 'a mut [T] > for & 'a mut GenericArray < T , N > { type Error = LengthError ; # [inline (always)] fn try_from (slice : & 'a mut [T]) -> Result < Self , Self :: Error > { GenericArray :: try_from_mut_slice (slice) } }
};
}
