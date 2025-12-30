// Generated macro for Ref (struct)
macro_rules! DepcrateRef {
() => {
// Module: crate
// Provides: {"Ref"}
// Dependencies: {}
# [doc = " An empty partial reference borrowing no parts."] # [doc = ""] # [doc = " Partial references with a non-empty set of borrowed parts are built by nesting this type within"] # [doc = " the [`Mut`] and [`Const`] types."] # [repr (transparent)] pub struct Ref < 'a , Target : PartialRefTarget + ? Sized > { ptr : * mut Target :: RawTarget , phantom : PhantomData < & 'a mut Target > , }
};
}
