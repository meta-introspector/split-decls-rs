// Generated macro for impl_713 (impl)
macro_rules! Depcrate_read_rnglistsimpl_713 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_713"}
// Dependencies: {}
impl < T > RangeLists < T > { # [doc = " Create a `RangeLists` that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `Dwarf::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> RangeLists < R > where F : FnMut (& 'a T) -> R , { RangeLists { debug_ranges : borrow (& self . debug_ranges . section) . into () , debug_rnglists : borrow (& self . debug_rnglists . section) . into () , } } }
};
}
