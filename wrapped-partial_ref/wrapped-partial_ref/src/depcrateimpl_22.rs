// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " Extending a valid reference by a mutable part is still a valid reference when the reference"] # [doc = " target has such a part."] impl < 'a , SomePart : Part , Reference : PartialRef < 'a > > PartialRef < 'a > for Mut < SomePart , Reference > where Reference :: Target : HasPart < SomePart > , { # [inline (always)] unsafe fn from_raw (ptr : * mut < Self :: Target as PartialRefTarget > :: RawTarget) -> Self { Mut { ptr , phantom : PhantomData , } } # [inline (always)] fn get_raw (& self) -> * mut < Self :: Target as PartialRefTarget > :: RawTarget { self . ptr } }
};
}
