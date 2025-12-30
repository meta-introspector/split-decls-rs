// Generated macro for FnOpaque (type)
macro_rules! Depcrate_parser_combinatorFnOpaque {
() => {
// Module: crate::parser::combinator
// Provides: {"FnOpaque"}
// Dependencies: {}
# [doc = " Alias over `Opaque` where the function can be a plain function pointer (does not need to"] # [doc = " capture any values)"] pub type FnOpaque < Input , O , S = () > = Opaque < fn (& mut dyn FnMut (& mut dyn Parser < Input , Output = O , PartialState = S >)) , Input , O , S > ;
};
}
