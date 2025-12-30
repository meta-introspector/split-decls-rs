// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl syn :: parse :: Parse for ImplementAttributes { fn parse (cursor : syn :: parse :: ParseStream) -> syn :: parse :: Result < Self > { let mut input = Self { agile : true , .. Default :: default () } ; while ! cursor . is_empty () { input . parse_implement (cursor) ? ; } Ok (input) } }
};
}
