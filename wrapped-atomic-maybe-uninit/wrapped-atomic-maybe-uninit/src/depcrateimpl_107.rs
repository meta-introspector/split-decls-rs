// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
impl < T : Primitive > From < T > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from an initialized value."] # [inline] fn from (v : T) -> Self { Self :: new (MaybeUninit :: new (v)) } }
};
}
