// Generated macro for impl_489 (impl)
macro_rules! Depcrate_read_indeximpl_489 {
() => {
// Module: crate::read::index
// Provides: {"impl_489"}
// Dependencies: {}
impl < T > DebugTuIndex < T > { # [doc = " Create a `DebugTuIndex` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfPackageSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugTuIndex < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
