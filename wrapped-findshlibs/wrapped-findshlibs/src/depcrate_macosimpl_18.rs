// Generated macro for impl_18 (impl)
macro_rules! Depcrate_macosimpl_18 {
() => {
// Module: crate::macos
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > SegmentTrait for Segment < 'a > { type SharedLibrary = SharedLibrary < 'a > ; # [inline] fn name (& self) -> & str { let cstr = match * self { Segment :: Segment32 (seg) => unsafe { CStr :: from_ptr (seg . segname . as_ptr ()) } , Segment :: Segment64 (seg) => unsafe { CStr :: from_ptr (seg . segname . as_ptr ()) } , } ; cstr . to_str () . unwrap_or ("(invalid segment name)") } # [inline] fn is_code (& self) -> bool { self . name () . as_bytes () == b"__TEXT" } # [inline] fn stated_virtual_memory_address (& self) -> Svma { match * self { Segment :: Segment32 (seg) => Svma (seg . vmaddr as usize) , Segment :: Segment64 (seg) => { assert ! (seg . vmaddr <= (usize :: MAX as u64)) ; Svma (seg . vmaddr as usize) } } } # [inline] fn len (& self) -> usize { match * self { Segment :: Segment32 (seg) => seg . vmsize as usize , Segment :: Segment64 (seg) => { assert ! (seg . vmsize <= (usize :: MAX as u64)) ; seg . vmsize as usize } } } }
};
}
