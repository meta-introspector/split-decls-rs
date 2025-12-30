// Generated macro for impl_738 (impl)
macro_rules! Depcrate_read_strimpl_738 {
() => {
// Module: crate::read::str
// Provides: {"impl_738"}
// Dependencies: {}
impl < T > DebugStr < T > { # [doc = " Create a `DebugStr` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugStr < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_str_section) . into () } }
};
}
