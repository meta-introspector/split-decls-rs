// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
# [doc = " Extending a valid reference by a constant part is still a valid reference when the reference"] # [doc = " target has such a part."] impl < 'a , SomePart : Part , Reference : PartialRef < 'a > > PartialRef < 'a > for Const < SomePart , Reference > where Reference :: Target : HasPart < SomePart > , { # [inline (always)] unsafe fn from_raw (ptr : * mut < Self :: Target as PartialRefTarget > :: RawTarget) -> Self { Const { ptr , phantom : PhantomData , } } # [inline (always)] fn get_raw (& self) -> * mut < Self :: Target as PartialRefTarget > :: RawTarget { self . ptr } }
};
}
