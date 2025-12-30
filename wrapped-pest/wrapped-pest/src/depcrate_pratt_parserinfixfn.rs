// Generated macro for InfixFn (type)
macro_rules! Depcrate_pratt_parserInfixFn {
() => {
// Module: crate::pratt_parser
// Provides: {"InfixFn"}
// Dependencies: {}
type InfixFn < 'a , 'i , R , T > = Box < dyn FnMut (T , Pair < 'i , R > , T) -> T + 'a > ;
};
}
