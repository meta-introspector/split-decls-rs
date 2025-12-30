// Generated macro for impl_513 (impl)
macro_rules! Depcrate_read_lineimpl_513 {
() => {
// Module: crate::read::line
// Provides: {"impl_513"}
// Dependencies: {}
impl < T > DebugLine < T > { # [doc = " Create a `DebugLine` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLine < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_line_section) . into () } }
};
}
