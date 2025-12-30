// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl Signature { # [doc = " Create a new Signature container from its components"] pub fn from_components (r : BoxedUint , s : BoxedUint) -> Option < Self > { let r = NonZero :: new (r) . into_option () ? ; let s = NonZero :: new (s) . into_option () ? ; Some (Self { r , s }) } # [doc = " Signature part r"] # [must_use] pub fn r (& self) -> & NonZero < BoxedUint > { & self . r } # [doc = " Signature part s"] # [must_use] pub fn s (& self) -> & NonZero < BoxedUint > { & self . s } }
};
}
