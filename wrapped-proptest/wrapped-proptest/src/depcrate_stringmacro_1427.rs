// Generated macro for macro_1427 (macro)
macro_rules! Depcrate_stringmacro_1427 {
() => {
// Module: crate::string
// Provides: {"macro_1427"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy which generates values (i.e., `String` or `Vec<u8>`) matching"] # [doc = " a regular expression."] # [doc = ""] # [doc = " Created by various functions in this module."] # [derive (Debug)] pub struct RegexGeneratorStrategy [< T >] [where T : fmt :: Debug] (SBoxedStrategy < T >) -> RegexGeneratorValueTree < T >; # [doc = " `ValueTree` corresponding to `RegexGeneratorStrategy`."] pub struct RegexGeneratorValueTree [< T >] [where T : fmt :: Debug] (Box < dyn ValueTree < Value = T >>) -> T ; }
};
}
