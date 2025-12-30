// Generated macro for UnionValueTree (struct)
macro_rules! Depcrate_strategy_unionsUnionValueTree {
() => {
// Module: crate::strategy::unions
// Provides: {"UnionValueTree"}
// Dependencies: {}
# [doc = " `ValueTree` corresponding to `Union`."] pub struct UnionValueTree < T : Strategy > { options : Vec < LazyValueTree < T > > , pick : usize , min_pick : usize , prev_pick : Option < usize > , }
};
}
