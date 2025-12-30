// Generated macro for impl_566 (impl)
macro_rules! Depcrate_read_loclistsimpl_566 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_566"}
// Dependencies: {}
impl < T > DebugLocLists < T > { # [doc = " Create a `DebugLocLists` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLocLists < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
