// Generated macro for impl_743 (impl)
macro_rules! Depcrate_read_strimpl_743 {
() => {
// Module: crate::read::str
// Provides: {"impl_743"}
// Dependencies: {}
impl < T > DebugStrOffsets < T > { # [doc = " Create a `DebugStrOffsets` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugStrOffsets < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
