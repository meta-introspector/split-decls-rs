// Generated macro for impl_106 (impl)
macro_rules! Depcrateimpl_106 {
() => {
// Module: crate
// Provides: {"impl_106"}
// Dependencies: {}
impl < T : Primitive > From < MaybeUninit < T > > for AtomicMaybeUninit < T > { # [doc = " Creates a new atomic value from a potentially uninitialized value."] # [inline] fn from (v : MaybeUninit < T >) -> Self { Self :: new (v) } }
};
}
