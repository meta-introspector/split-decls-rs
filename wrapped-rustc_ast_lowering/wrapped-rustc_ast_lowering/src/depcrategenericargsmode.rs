// Generated macro for GenericArgsMode (enum)
macro_rules! DepcrateGenericArgsMode {
() => {
// Module: crate
// Provides: {"GenericArgsMode"}
// Dependencies: {}
enum GenericArgsMode { # [doc = " Allow paren sugar, don't allow RTN."] ParenSugar , # [doc = " Allow RTN, don't allow paren sugar."] ReturnTypeNotation , Err , # [doc = " Silence errors when lowering generics. Only used with `Res::Err`."] Silence , }
};
}
