// Generated macro for impl_92 (impl)
macro_rules! Depcrate_windowsimpl_92 {
() => {
// Module: crate::windows
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a > SegmentTrait for Segment < 'a > { type SharedLibrary = SharedLibrary < 'a > ; # [inline] fn name (& self) -> & str { std :: str :: from_utf8 (& self . section . Name) . unwrap_or ("") . trim_end_matches ('\0') } fn is_code (& self) -> bool { (self . section . Characteristics & IMAGE_SCN_CNT_CODE) != 0 } # [inline] fn stated_virtual_memory_address (& self) -> Svma { Svma (self . section . VirtualAddress as usize) } # [inline] fn len (& self) -> usize { * unsafe { self . section . Misc . VirtualSize () } as usize } }
};
}
