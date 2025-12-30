// Generated macro for Tokens (struct)
macro_rules! Depcrate_iterators_tokensTokens {
() => {
// Module: crate::iterators::tokens
// Provides: {"Tokens"}
// Dependencies: {}
# [doc = " An iterator over [`Token`]s. It is created by [`Pair::tokens`] and [`Pairs::tokens`]."] # [doc = ""] # [doc = " [`Token`]: ../enum.Token.html"] # [doc = " [`Pair::tokens`]: struct.Pair.html#method.tokens"] # [doc = " [`Pairs::tokens`]: struct.Pairs.html#method.tokens"] # [derive (Clone)] pub struct Tokens < 'i , R > { queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , start : usize , end : usize , }
};
}
