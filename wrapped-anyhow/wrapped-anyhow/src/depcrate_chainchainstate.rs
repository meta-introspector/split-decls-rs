// Generated macro for ChainState (enum)
macro_rules! Depcrate_chainChainState {
() => {
// Module: crate::chain
// Provides: {"ChainState"}
// Dependencies: {}
# [derive (Clone)] pub (crate) enum ChainState < 'a > { Linked { next : Option < & 'a (dyn StdError + 'static) > , } , # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] Buffered { rest : vec :: IntoIter < & 'a (dyn StdError + 'static) > , } , }
};
}
