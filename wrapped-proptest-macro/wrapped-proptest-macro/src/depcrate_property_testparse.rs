// Generated macro for parse (macro)
macro_rules! Depcrate_property_testparse {
() => {
// Module: crate::property_test
// Provides: {"parse"}
// Dependencies: {}
# [doc = " try to parse an item, or return the error as a token stream"] macro_rules ! parse { ($ e : expr) => { match parse2 ($ e) { Ok (item) => item , Err (e) => return e . into_compile_error () , } } ; }
};
}
