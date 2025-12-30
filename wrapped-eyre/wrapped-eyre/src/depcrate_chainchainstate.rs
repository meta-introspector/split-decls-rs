// Generated macro for ChainState (enum)
macro_rules! Depcrate_chainChainState {
() => {
// Module: crate::chain
// Provides: {"ChainState"}
// Dependencies: {}
# [derive (Clone)] pub (crate) enum ChainState < 'a > { Linked { next : Option < & 'a (dyn StdError + 'static) > , } , Buffered { rest : vec :: IntoIter < & 'a (dyn StdError + 'static) > , } , }
};
}
