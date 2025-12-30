// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl syn :: parse :: Parse for Guid { fn parse (cursor : syn :: parse :: ParseStream) -> syn :: Result < Self > { let string : Option < syn :: LitStr > = cursor . parse () . ok () ; Ok (Self (string)) } }
};
}
