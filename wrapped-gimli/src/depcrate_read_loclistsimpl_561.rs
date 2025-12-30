// Generated macro for impl_561 (impl)
macro_rules! Depcrate_read_loclistsimpl_561 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_561"}
// Dependencies: {}
impl < T > DebugLoc < T > { # [doc = " Create a `DebugLoc` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLoc < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
