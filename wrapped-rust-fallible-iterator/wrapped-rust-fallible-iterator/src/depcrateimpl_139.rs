// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl < I > IteratorExt for I where I : iter :: Iterator , { # [doc = " Convert an iterator of `Result`s into `FallibleIterator` by transposition"] fn transpose_into_fallible < T , E > (self) -> Convert < Self > where Self : iter :: Iterator < Item = Result < T , E > > + Sized , { Convert (self) } # [doc = " Convert an iterator of anything into `FallibleIterator` by wrapping"] # [doc = " into `Result<T, Infallible>` where `Infallible` is an error that can never actually"] # [doc = " happen."] fn into_fallible < T > (self) -> IntoFallible < Self > where Self : iter :: Iterator < Item = T > + Sized , { IntoFallible (self) } }
};
}
