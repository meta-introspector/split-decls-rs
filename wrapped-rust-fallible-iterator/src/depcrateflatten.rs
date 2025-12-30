// Generated macro for Flatten (struct)
macro_rules! DepcrateFlatten {
() => {
// Module: crate
// Provides: {"Flatten"}
// Dependencies: {}
# [doc = " An iterator which flattens an iterator of iterators, yielding those iterators' elements."] pub struct Flatten < I > where I : FallibleIterator , I :: Item : IntoFallibleIterator , { it : I , cur : Option < < I :: Item as IntoFallibleIterator > :: IntoFallibleIter > , }
};
}
