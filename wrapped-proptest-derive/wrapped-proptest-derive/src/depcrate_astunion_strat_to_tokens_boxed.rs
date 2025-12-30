// Generated macro for union_strat_to_tokens_boxed (function)
macro_rules! Depcrate_astunion_strat_to_tokens_boxed {
() => {
// Module: crate::ast
// Provides: {"union_strat_to_tokens_boxed"}
// Dependencies: {}
# [doc = " Tokenizes a weighted list of `Strategy`."] # [doc = " For details, see `union_ctor_to_tokens_boxed`."] # [cfg (feature = "boxed_union")] fn union_strat_to_tokens_boxed (tokens : & mut TokenStream , strats : & [Strategy]) { if strats . is_empty () { return ; } if let [strat] = strats { strat . to_tokens (tokens) ; return ; } quote_append ! (tokens , _proptest :: strategy :: Union < _proptest :: strategy :: BoxedStrategy < Self >>) ; }
};
}
