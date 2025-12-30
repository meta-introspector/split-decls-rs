// Generated macro for Chain (struct)
macro_rules! Depcrate_stream_chain_vecChain {
() => {
// Module: crate::stream::chain::vec
// Provides: {"Chain"}
// Dependencies: {}
# [doc = " A stream that chains multiple streams one after another."] # [doc = ""] # [doc = " This `struct` is created by the [`chain`] method on the [`Chain`] trait. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`chain`]: trait.Chain.html#method.merge"] # [doc = " [`Chain`]: trait.Chain.html"] # [pin_project] pub struct Chain < S > { # [pin] streams : Vec < S > , index : usize , len : usize , done : bool , }
};
}
