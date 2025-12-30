// Generated macro for impl_173 (impl)
macro_rules! Depcrate_read_addrimpl_173 {
() => {
// Module: crate::read::addr
// Provides: {"impl_173"}
// Dependencies: {}
impl < T > DebugAddr < T > { # [doc = " Create a `DebugAddr` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugAddr < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
};
}
