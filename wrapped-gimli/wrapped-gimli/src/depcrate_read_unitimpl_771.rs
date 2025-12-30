// Generated macro for impl_771 (impl)
macro_rules! Depcrate_read_unitimpl_771 {
() => {
// Module: crate::read::unit
// Provides: {"impl_771"}
// Dependencies: {}
impl < T > DebugInfo < T > { # [doc = " Create a `DebugInfo` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugInfo < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_info_section) . into () } }
};
}
