// Generated macro for impl_460 (impl)
macro_rules! Depcrate_read_arangesimpl_460 {
() => {
// Module: crate::read::aranges
// Provides: {"impl_460"}
// Dependencies: {}
impl < T > DebugAranges < T > { # [doc = " Create a `DebugAranges` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugAranges < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
