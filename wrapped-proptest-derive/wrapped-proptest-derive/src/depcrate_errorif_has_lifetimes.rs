// Generated macro for if_has_lifetimes (function)
macro_rules! Depcrate_errorif_has_lifetimes {
() => {
// Module: crate::error
// Provides: {"if_has_lifetimes"}
// Dependencies: {}
# [doc = " Ensures that the type is not parametric over lifetimes."] pub fn if_has_lifetimes (ctx : Ctx , ast : & syn :: DeriveInput) { if ast . generics . lifetimes () . count () > 0 { has_lifetimes (ctx) ; } }
};
}
