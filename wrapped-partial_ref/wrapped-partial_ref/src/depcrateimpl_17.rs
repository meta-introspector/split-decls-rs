// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [doc = " An empty reference to a valid target is a valid reference."] impl < 'a , 'b : 'a , Target : PartialRefTarget + ? Sized > PartialRef < 'a > for Ref < 'b , Target > { # [inline (always)] unsafe fn from_raw (ptr : * mut < Self :: Target as PartialRefTarget > :: RawTarget) -> Self { Ref { ptr , phantom : PhantomData , } } # [inline (always)] fn get_raw (& self) -> * mut < Self :: Target as PartialRefTarget > :: RawTarget { self . ptr } }
};
}
