// Generated macro for impl_171 (impl)
macro_rules! Depcrate_arcimpl_171 {
() => {
// Module: crate::arc
// Provides: {"impl_171"}
// Dependencies: {}
impl < H , T > Deref for ThinArc < H , T > { type Target = HeaderSlice < H , [T] > ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { & (* thin_to_thick (self . ptr . as_ptr ())) . data } } }
};
}
