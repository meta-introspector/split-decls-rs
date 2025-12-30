// Generated macro for IteratorExt (trait)
macro_rules! DepcrateIteratorExt {
() => {
// Module: crate
// Provides: {"IteratorExt"}
// Dependencies: {}
# [doc = " An extnsion-trait with set of useful methods to convert [`core::iter::Iterator`]"] # [doc = " into [`FallibleIterator`]"] pub trait IteratorExt { # [doc = " Convert an iterator of `Result`s into `FallibleIterator` by transposition"] fn transpose_into_fallible < T , E > (self) -> Convert < Self > where Self : iter :: Iterator < Item = Result < T , E > > + Sized ; # [doc = " Convert an iterator of anything into `FallibleIterator` by wrapping"] # [doc = " into `Result<T, Infallible>` where `Infallible` is an error that can never actually"] # [doc = " happen."] fn into_fallible < T > (self) -> IntoFallible < Self > where Self : iter :: Iterator < Item = T > + Sized ; }
};
}
