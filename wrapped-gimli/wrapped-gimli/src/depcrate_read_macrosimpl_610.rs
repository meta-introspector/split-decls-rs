// Generated macro for impl_610 (impl)
macro_rules! Depcrate_read_macrosimpl_610 {
() => {
// Module: crate::read::macros
// Provides: {"impl_610"}
// Dependencies: {}
impl < T > DebugMacinfo < T > { # [doc = " Create a `DebugMacinfo` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugMacinfo < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
