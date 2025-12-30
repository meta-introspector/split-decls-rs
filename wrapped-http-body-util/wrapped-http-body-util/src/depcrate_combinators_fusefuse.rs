// Generated macro for Fuse (struct)
macro_rules! Depcrate_combinators_fuseFuse {
() => {
// Module: crate::combinators::fuse
// Provides: {"Fuse"}
// Dependencies: {}
# [doc = " A \"fused\" [`Body`]."] # [doc = ""] # [doc = " This [`Body`] yields `Poll::Ready(None)` forever after the underlying body yields"] # [doc = " `Poll::Ready(None)`, or an error `Poll::Ready(Some(Err(_)))`, once."] # [doc = ""] # [doc = " Bodies should ideally continue to return `Poll::Ready(None)` indefinitely after the end of"] # [doc = " the stream is reached. [`Fuse<B>`] avoids polling its underlying body `B` further after the"] # [doc = " underlying stream as ended, which can be useful for implementation that cannot uphold this"] # [doc = " guarantee."] # [doc = ""] # [doc = " This is akin to the functionality that [`std::iter::Iterator::fuse()`] provides for"] # [doc = " [`Iterator`][std::iter::Iterator]s."] # [derive (Debug)] pub struct Fuse < B > { inner : Option < B > , }
};
}
