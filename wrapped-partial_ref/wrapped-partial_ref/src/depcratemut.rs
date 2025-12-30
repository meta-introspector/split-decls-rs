// Generated macro for Mut (struct)
macro_rules! DepcrateMut {
() => {
// Module: crate
// Provides: {"Mut"}
// Dependencies: {}
# [doc = " A mutable part of a partial reference."] # [repr (transparent)] pub struct Mut < Part , Reference : HasTarget > { ptr : * mut < < Reference as HasTarget > :: Target as PartialRefTarget > :: RawTarget , phantom : PhantomData < (Reference , Part) > , }
};
}
