// Generated macro for thin_to_thick (function)
macro_rules! Depcrate_arcthin_to_thick {
() => {
// Module: crate::arc
// Provides: {"thin_to_thick"}
// Dependencies: {}
fn thin_to_thick < H , T > (thin : * mut ArcInner < HeaderSlice < H , [T ; 0] > > ,) -> * mut ArcInner < HeaderSlice < H , [T] > > { let len = unsafe { (* thin) . data . length } ; let fake_slice : * mut [T] = ptr :: slice_from_raw_parts_mut (thin as * mut T , len) ; fake_slice as * mut ArcInner < HeaderSlice < H , [T] > > }
};
}
