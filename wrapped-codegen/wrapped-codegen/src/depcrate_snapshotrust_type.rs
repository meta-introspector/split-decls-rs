// Generated macro for rust_type (function)
macro_rules! Depcrate_snapshotrust_type {
() => {
// Module: crate::snapshot
// Provides: {"rust_type"}
// Dependencies: {}
fn rust_type (ty : & Type) -> TokenStream { match ty { Type :: Syn (ty) => { let ident = Ident :: new (ty , Span :: call_site ()) ; quote ! (syn ::# ident) } Type :: Std (ty) => { let ident = Ident :: new (ty , Span :: call_site ()) ; quote ! (# ident) } Type :: Ext (ty) => { let ident = Ident :: new (ty , Span :: call_site ()) ; quote ! (proc_macro2 ::# ident) } Type :: Token (ty) | Type :: Group (ty) => { let ident = Ident :: new (ty , Span :: call_site ()) ; quote ! (syn :: token ::# ident) } Type :: Punctuated (ty) => { let element = rust_type (& ty . element) ; let punct = Ident :: new (& ty . punct , Span :: call_site ()) ; quote ! (syn :: punctuated :: Punctuated <# element , # punct >) } Type :: Option (ty) => { let inner = rust_type (ty) ; quote ! (Option <# inner >) } Type :: Box (ty) => { let inner = rust_type (ty) ; quote ! (Box <# inner >) } Type :: Vec (ty) => { let inner = rust_type (ty) ; quote ! (Vec <# inner >) } Type :: Tuple (ty) => { let inner = ty . iter () . map (rust_type) ; quote ! ((# (# inner ,) *)) } } }
};
}
