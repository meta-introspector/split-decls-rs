// Generated macro for tuple_example_impls (macro)
macro_rules! Depcrate_abi_exampletuple_example_impls {
() => {
// Module: crate::abi_example
// Provides: {"tuple_example_impls"}
// Dependencies: {}
macro_rules ! tuple_example_impls { ($ ($ Tuple : ident { $ (($ idx : tt) -> $ T : ident) + }) +) => { $ (impl <$ ($ T : AbiExample) ,+> AbiExample for ($ ($ T ,) +) { fn example () -> Self { ($ ({ let x : $ T = AbiExample :: example () ; x } ,) +) } }) + } }
};
}
