// Generated macro for impl_429 (impl)
macro_rules! Depcrate_read_abbrevimpl_429 {
() => {
// Module: crate::read::abbrev
// Provides: {"impl_429"}
// Dependencies: {}
impl < T > DebugAbbrev < T > { # [doc = " Create a `DebugAbbrev` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugAbbrev < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_abbrev_section) . into () } }
};
}
