// Generated macro for Opaque (struct)
macro_rules! DepcrateOpaque {
() => {
// Module: crate
// Provides: {"Opaque"}
// Dependencies: {}
# [doc = " An opaque type used to define `ForeignTypeRef` types."] # [doc = ""] # [doc = " A type implementing `ForeignTypeRef` should simply be a newtype wrapper around this type."] pub struct Opaque (PhantomData < UnsafeCell < * mut () > >) ;
};
}
