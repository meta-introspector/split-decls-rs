// Generated macro for impl_7 (impl)
macro_rules! Depcrate_internalimpl_7 {
() => {
// Module: crate::internal
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : ? Sized > CastToken < T > { # [doc = " Create a cast token for the given type of value."] pub const fn of_val (_value : & T) -> Self { Self :: of () } # [doc = " Create a new cast token of the specified type."] pub const fn of () -> Self { Self (PhantomData) } }
};
}
