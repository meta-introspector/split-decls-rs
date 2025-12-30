// Generated macro for doit (macro)
macro_rules! Depcrate_netdoit {
() => {
// Module: crate::net
// Provides: {"doit"}
// Dependencies: {}
macro_rules ! doit { ($ ($ t : ident) *) => ($ (impl NetInt for $ t { fn from_be (i : Self) -> Self { <$ t >:: from_be (i) } }) *) }
};
}
