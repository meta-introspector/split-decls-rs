// Generated macro for impl_701 (impl)
macro_rules! Depcrate_read_rnglistsimpl_701 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_701"}
// Dependencies: {}
impl < T > DebugRanges < T > { # [doc = " Create a `DebugRanges` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugRanges < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
