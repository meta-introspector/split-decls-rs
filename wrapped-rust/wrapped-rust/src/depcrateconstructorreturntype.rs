// Generated macro for ConstructorReturnType (enum)
macro_rules! DepcrateConstructorReturnType {
() => {
// Module: crate
// Provides: {"ConstructorReturnType"}
// Dependencies: {}
enum ConstructorReturnType { # [doc = " Resource constructor is infallible. E.g.:"] # [doc = " ```wit"] # [doc = " resource R {"] # [doc = "    constructor(..);"] # [doc = " }"] # [doc = " ```"] Self_ , # [doc = " Resource constructor is fallible. E.g.:"] # [doc = " ```wit"] # [doc = " resource R {"] # [doc = "    constructor(..) -> result<R, err>;"] # [doc = " }"] # [doc = " ```"] Result { err : Option < Type > } , }
};
}
