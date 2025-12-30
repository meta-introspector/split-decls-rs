// Generated macro for impl_573 (impl)
macro_rules! Depcrate_read_loclistsimpl_573 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_573"}
// Dependencies: {}
impl < T > LocationLists < T > { # [doc = " Create a `LocationLists` that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `Dwarf::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> LocationLists < R > where F : FnMut (& 'a T) -> R , { LocationLists { debug_loc : borrow (& self . debug_loc . section) . into () , debug_loclists : borrow (& self . debug_loclists . section) . into () , } } }
};
}
