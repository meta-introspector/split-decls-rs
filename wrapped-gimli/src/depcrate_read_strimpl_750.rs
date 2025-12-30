// Generated macro for impl_750 (impl)
macro_rules! Depcrate_read_strimpl_750 {
() => {
// Module: crate::read::str
// Provides: {"impl_750"}
// Dependencies: {}
impl < T > DebugLineStr < T > { # [doc = " Create a `DebugLineStr` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLineStr < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
