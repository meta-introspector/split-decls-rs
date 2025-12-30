// Generated macro for build_ctor_args (function)
macro_rules! Depcrate_item_typebuild_ctor_args {
() => {
// Module: crate::item_type
// Provides: {"build_ctor_args"}
// Dependencies: {}
fn build_ctor_args (fields : & Fields , values : & [impl ToTokens]) -> TokenStream { match fields { Fields :: Named (fields) => { let names = fields . named . iter () . map (| f | f . ident . as_ref () . unwrap ()) ; quote ! ({ # (# names : # values ,) * }) } Fields :: Unnamed (_) => quote ! ((# (# values ,) *)) , Fields :: Unit => quote ! () , } }
};
}
