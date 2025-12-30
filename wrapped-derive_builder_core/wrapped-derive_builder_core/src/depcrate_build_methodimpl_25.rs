// Generated macro for impl_25 (impl)
macro_rules! Depcrate_build_methodimpl_25 {
() => {
// Module: crate::build_method
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > ToTokens for BuildMethod < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let ident = & self . ident ; let vis = & self . visibility ; let target_ty = & self . target_ty ; let target_ty_generics = & self . target_ty_generics ; let initializers = & self . initializers ; let self_param = match self . pattern { BuilderPattern :: Owned => quote ! (self) , BuilderPattern :: Mutable | BuilderPattern :: Immutable => quote ! (& self) , } ; let doc_comment = & self . doc_comment ; let default_struct = self . default_struct . as_ref () . map (| default_expr | { let default_expr = default_expr . with_crate_root (self . crate_root) ; let ident = syn :: Ident :: new (DEFAULT_STRUCT_NAME , Span :: call_site ()) ; quote ! (let # ident : # target_ty # target_ty_generics = # default_expr ;) }) ; let validate_fn = self . validate_fn . as_ref () . map (| vfn | quote_spanned ! (vfn . span () => # vfn (& self) ?;)) ; let error_ty = & self . error_ty ; if self . enabled { let crate_root = & self . crate_root ; tokens . append_all (quote ! (# doc_comment # vis fn # ident (# self_param) -> # crate_root :: export :: core :: result :: Result <# target_ty # target_ty_generics , # error_ty > { # validate_fn # default_struct Ok (# target_ty { # (# initializers) * }) })) } } }
};
}
