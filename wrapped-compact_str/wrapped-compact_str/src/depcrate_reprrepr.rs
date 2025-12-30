// Generated macro for Repr (struct)
macro_rules! Depcrate_reprRepr {
() => {
// Module: crate::repr
// Provides: {"Repr"}
// Dependencies: {}
# [repr (C)] pub (crate) struct Repr (# [doc = " We have a pointer in the representation to properly carry provenance."] * const () , # [doc = " Then we need two `usize`s (aka WORDs) of data, for the first we just define a `usize`..."] usize , # [doc = " ...but the second we breakup into multiple pieces..."] # [cfg (target_pointer_width = "64")] u32 , u16 , u8 , # [doc = " ...so that the last byte can be a [`LastByte`], which allows the compiler to see a niche"] # [doc = " value."] LastByte ,) ;
};
}
