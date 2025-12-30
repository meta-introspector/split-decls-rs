// Generated macro for impl_tuple (macro)
macro_rules! Depcrate_arbitrary_tuplesimpl_tuple {
() => {
// Module: crate::arbitrary::tuples
// Provides: {"impl_tuple"}
// Dependencies: {}
macro_rules ! impl_tuple { ($ ($ typ : ident) ,*) => { impl <$ ($ typ : Arbitrary) ,*> Arbitrary for ($ ($ typ ,) *) { type Parameters = product_type ! [$ ($ typ :: Parameters ,) *] ; type Strategy = ($ ($ typ :: Strategy ,) *) ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { # [allow (non_snake_case)] let product_unpack ! [$ ($ typ) ,*] = args ; ($ (any_with ::<$ typ > ($ typ)) ,*,) } } } ; }
};
}
