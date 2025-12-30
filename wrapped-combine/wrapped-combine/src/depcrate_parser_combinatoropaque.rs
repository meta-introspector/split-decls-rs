// Generated macro for opaque (macro)
macro_rules! Depcrate_parser_combinatoropaque {
() => {
// Module: crate::parser::combinator
// Provides: {"opaque"}
// Dependencies: {}
# [doc = " Convenience macro over [`opaque`][]."] # [doc = ""] # [doc = " [`opaque`]: parser/combinator/fn.opaque.html"] # [macro_export] macro_rules ! opaque { ($ e : expr) => { $ crate :: opaque ! ($ e ,) ; } ; ($ e : expr ,) => { $ crate :: parser :: combinator :: opaque (move | f : & mut dyn FnMut (& mut $ crate :: Parser < _ , Output = _ , PartialState = _ >) | { f (& mut $ e) } ,) } ; }
};
}
