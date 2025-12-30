// Generated macro for PostfixFn (type)
macro_rules! Depcrate_pratt_parserPostfixFn {
() => {
// Module: crate::pratt_parser
// Provides: {"PostfixFn"}
// Dependencies: {}
type PostfixFn < 'a , 'i , R , T > = Box < dyn FnMut (T , Pair < 'i , R >) -> T + 'a > ;
};
}
