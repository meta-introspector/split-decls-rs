// Generated macro for impl_75 (impl)
macro_rules! Depcrate_header_mapimpl_75 {
() => {
// Module: crate::header::map
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = (& 'a HeaderName , & 'a mut T) ; fn next (& mut self) -> Option < Self :: Item > { self . next_unsafe () . map (| (key , ptr) | (key , unsafe { & mut * ptr })) } fn size_hint (& self) -> (usize , Option < usize >) { let map = unsafe { & * self . map } ; debug_assert ! (map . entries . len () >= self . entry) ; let lower = map . entries . len () - self . entry ; (lower , None) } }
};
}
