// Generated macro for impl_239 (impl)
macro_rules! Depcrateimpl_239 {
() => {
// Module: crate
// Provides: {"impl_239"}
// Dependencies: {}
impl SsrMatches { # [doc = " Returns `self` with any nested matches removed and made into top-level matches."] pub fn flattened (self) -> SsrMatches { let mut out = SsrMatches :: default () ; self . flatten_into (& mut out) ; out } fn flatten_into (self , out : & mut SsrMatches) { for mut m in self . matches { for p in m . placeholder_values . values_mut () { std :: mem :: take (& mut p . inner_matches) . flatten_into (out) ; } out . matches . push (m) ; } } }
};
}
