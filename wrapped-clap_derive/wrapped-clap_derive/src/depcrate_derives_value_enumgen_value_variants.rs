// Generated macro for gen_value_variants (function)
macro_rules! Depcrate_derives_value_enumgen_value_variants {
() => {
// Module: crate::derives::value_enum
// Provides: {"gen_value_variants"}
// Dependencies: {}
fn gen_value_variants (lits : & [(TokenStream , Ident)]) -> TokenStream { let lit = lits . iter () . map (| l | & l . 1) . collect :: < Vec < _ > > () ; quote ! { fn value_variants <'a > () -> &'a [Self] { & [# (Self ::# lit) ,*] } } }
};
}
