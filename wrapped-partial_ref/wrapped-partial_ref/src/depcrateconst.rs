// Generated macro for Const (struct)
macro_rules! DepcrateConst {
() => {
// Module: crate
// Provides: {"Const"}
// Dependencies: {}
# [doc = " A constant (immutable) part of a partial reference."] # [repr (transparent)] pub struct Const < Part , Reference : HasTarget > { ptr : * mut < < Reference as HasTarget > :: Target as PartialRefTarget > :: RawTarget , phantom : PhantomData < (Reference , Part) > , }
};
}
