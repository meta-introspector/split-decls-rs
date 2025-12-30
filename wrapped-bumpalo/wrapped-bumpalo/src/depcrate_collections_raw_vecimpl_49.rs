// Generated macro for impl_49 (impl)
macro_rules! Depcrate_collections_raw_vecimpl_49 {
() => {
// Module: crate::collections::raw_vec
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , T > RawVec < 'a , T > { # [doc = " Reconstitutes a RawVec from a pointer, capacity, and allocator."] # [doc = ""] # [doc = " # Undefined Behavior"] # [doc = ""] # [doc = " The ptr must be allocated (via the given allocator `a`), and with the given capacity. The"] # [doc = " capacity cannot exceed `isize::MAX` (only a concern on 32-bit systems)."] # [doc = " If the ptr and capacity come from a RawVec created via `a`, then this is guaranteed."] pub unsafe fn from_raw_parts_in (ptr : * mut T , cap : usize , a : & 'a Bump) -> Self { RawVec { ptr : NonNull :: new_unchecked (ptr) , cap , a , } } }
};
}
