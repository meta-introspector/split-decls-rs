// Generated macro for impl_706 (impl)
macro_rules! Depcrate_read_rnglistsimpl_706 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_706"}
// Dependencies: {}
impl < T > DebugRngLists < T > { # [doc = " Create a `DebugRngLists` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugRngLists < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
