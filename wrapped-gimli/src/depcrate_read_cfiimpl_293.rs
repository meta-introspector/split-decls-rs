// Generated macro for impl_293 (impl)
macro_rules! Depcrate_read_cfiimpl_293 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_293"}
// Dependencies: {}
impl Pointer { # [inline] fn new (encoding : constants :: DwEhPe , address : u64) -> Pointer { if encoding . is_indirect () { Pointer :: Indirect (address) } else { Pointer :: Direct (address) } } # [doc = " Return the direct pointer value."] # [inline] pub fn direct (self) -> Result < u64 > { match self { Pointer :: Direct (p) => Ok (p) , Pointer :: Indirect (_) => Err (Error :: UnsupportedPointerEncoding) , } } # [doc = " Return the pointer value, discarding indirectness information."] # [inline] pub fn pointer (self) -> u64 { match self { Pointer :: Direct (p) | Pointer :: Indirect (p) => p , } } }
};
}
