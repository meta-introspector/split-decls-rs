// Generated macro for FlatMap (struct)
macro_rules! DepcrateFlatMap {
() => {
// Module: crate
// Provides: {"FlatMap"}
// Dependencies: {}
# [doc = " An iterator which maps each element to another iterator, yielding those iterator's elements."] # [derive (Clone , Debug)] pub struct FlatMap < I , U , F > where U : IntoFallibleIterator , { it : Map < I , F > , cur : Option < U :: IntoFallibleIter > , }
};
}
