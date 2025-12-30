// Generated macro for union_strat_to_tokens (function)
macro_rules! Depcrate_astunion_strat_to_tokens {
() => {
// Module: crate::ast
// Provides: {"union_strat_to_tokens"}
// Dependencies: {}
# [doc = " Tokenizes a weighted list of `Strategy`."] # [doc = " For details, see `union_ctor_to_tokens`."] # [cfg (not (feature = "boxed_union"))] fn union_strat_to_tokens (tokens : & mut TokenStream , strats : & [Strategy]) { if strats . is_empty () { return ; } if let [strat] = strats { strat . to_tokens (tokens) ; return ; } let mut chunks = strats . chunks (UNION_CHUNK_SIZE) ; let chunk = chunks . next () . unwrap () ; let head = chunk . iter () . map (wrap_arc) ; let tail = Recurse (chunks) ; quote_append ! (tokens , _proptest :: strategy :: TupleUnion < (# (# head ,) * # tail) >) ; struct Recurse < 'a > (:: std :: slice :: Chunks < 'a , Strategy >) ; impl < 'a > ToTokens for Recurse < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let mut chunks = self . 0 . clone () ; if let Some (chunk) = chunks . next () { if let [s] = chunk { quote_append ! (tokens , (u32 , :: std :: sync :: Arc <# s >)) ; } else { let head = chunk . iter () . map (wrap_arc) ; let tail = Recurse (chunks) ; quote_append ! (tokens , (u32 , :: std :: sync :: Arc < _proptest :: strategy :: TupleUnion < (# (# head ,) * # tail) >>)) ; } } } } fn wrap_arc (s : & Strategy) -> TokenStream { quote ! ((u32 , :: std :: sync :: Arc <# s >)) } }
};
}
