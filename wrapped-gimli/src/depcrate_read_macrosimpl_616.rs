// Generated macro for impl_616 (impl)
macro_rules! Depcrate_read_macrosimpl_616 {
() => {
// Module: crate::read::macros
// Provides: {"impl_616"}
// Dependencies: {}
impl < T > DebugMacro < T > { # [doc = " Create a `DebugMacro` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugMacro < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
