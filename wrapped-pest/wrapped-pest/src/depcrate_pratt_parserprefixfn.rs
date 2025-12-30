// Generated macro for PrefixFn (type)
macro_rules! Depcrate_pratt_parserPrefixFn {
() => {
// Module: crate::pratt_parser
// Provides: {"PrefixFn"}
// Dependencies: {}
type PrefixFn < 'a , 'i , R , T > = Box < dyn FnMut (Pair < 'i , R > , T) -> T + 'a > ;
};
}
