// Generated macro for impl_483 (impl)
macro_rules! Depcrate_read_indeximpl_483 {
() => {
// Module: crate::read::index
// Provides: {"impl_483"}
// Dependencies: {}
impl < T > DebugCuIndex < T > { # [doc = " Create a `DebugCuIndex` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfPackageSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugCuIndex < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
