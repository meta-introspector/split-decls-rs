// Generated macro for impl_658 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_658 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_658"}
// Dependencies: {}
# [doc = " Creates an empty iterator."] impl < 'a , T : 'a > Default for ValuesRef < 'a , T > { fn default () -> Self { static EMPTY : [Vec < AnyValue > ; 0] = [] ; ValuesRef { iter : EMPTY [..] . iter () . flatten () . map (| _ | unreachable ! ()) , len : 0 , } } }
};
}
