// Generated macro for impl_100 (impl)
macro_rules! Depcrate_item_typeimpl_100 {
() => {
// Module: crate::item_type
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a > VariantEntry < 'a > { fn new (variant : & 'a Variant , kinds : & HelperAttributeKinds) -> Result < Self > { Ok (Self { variant , fields : FieldEntry :: from_fields (& variant . fields , kinds) ? , hattrs : HelperAttributes :: from_attrs (& variant . attrs , AttributeTarget :: Variant , kinds) ? , }) } fn from_variants (variants : impl IntoIterator < Item = & 'a Variant > , kinds : & HelperAttributeKinds ,) -> Result < Vec < Self > > { variants . into_iter () . map (| variant | Self :: new (variant , kinds)) . collect () } fn make_pat (& self , prefix : & str) -> TokenStream { self . make_pat_with_self_path (prefix , quote ! (Self)) } fn make_pat_with_self_path (& self , prefix : & str , self_path : impl ToTokens) -> TokenStream { let ident = & self . variant . ident ; let mut args = Vec :: new () ; for field in & self . fields { args . push (field . make_ident (prefix)) ; } let args = build_ctor_args (& self . variant . fields , & args) ; quote ! (# self_path ::# ident # args) } fn make_pat_wildcard (& self) -> TokenStream { let ident = & self . variant . ident ; let args = match & self . variant . fields { Fields :: Named (_) => quote ! ({ .. }) , Fields :: Unnamed (_) => quote ! ((..)) , Fields :: Unit => quote ! () , } ; quote ! (Self ::# ident # args) } }
};
}
