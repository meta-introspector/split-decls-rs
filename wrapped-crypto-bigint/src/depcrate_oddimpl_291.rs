// Generated macro for impl_291 (impl)
macro_rules! Depcrate_oddimpl_291 {
() => {
// Module: crate::odd
// Provides: {"impl_291"}
// Dependencies: {}
impl Odd < UintRef > { # [doc = " Construct an [`Odd<Uint<T>>`] from the unsigned integer value,"] # [doc = " truncating the upper bits if the value is too large to be"] # [doc = " represented."] pub const fn to_uint_resize < const T : usize > (& self) -> Odd < Uint < T > > { Odd (self . 0 . to_uint_resize ()) } }
};
}
