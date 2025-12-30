// Generated macro for impl_165 (impl)
macro_rules! Depcrate_arcimpl_165 {
() => {
// Module: crate::arc
// Provides: {"impl_165"}
// Dependencies: {}
impl < H , T > Deref for HeaderSlice < H , [T ; 0] > { type Target = HeaderSlice < H , [T] > ; fn deref (& self) -> & Self :: Target { let len = self . length ; let fake_slice : * const [T] = ptr :: slice_from_raw_parts (self as * const _ as * const T , len) ; unsafe { & * (fake_slice as * const HeaderSlice < H , [T] >) } } }
};
}
