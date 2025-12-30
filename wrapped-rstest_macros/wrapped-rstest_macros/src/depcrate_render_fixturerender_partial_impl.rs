// Generated macro for render_partial_impl (function)
macro_rules! Depcrate_render_fixturerender_partial_impl {
() => {
// Module: crate::render::fixture
// Provides: {"render_partial_impl"}
// Dependencies: {}
fn render_partial_impl (args : & [FnArg] , output : & ReturnType , generics : & Generics , asyncness : Option < & Async > , n : usize , resolver : & impl Resolver , info : & FixtureInfo ,) -> TokenStream { let mut output = info . attributes . extract_partial_type (n) . unwrap_or_else (| | output . clone ()) ; if info . arguments . is_once () { output = wrap_return_type_as_static_ref (output) ; } let generics = generics_clean_up (generics , args . iter () . take (n) , & output) ; let where_clause = & generics . where_clause ; let genercs_idents = generics . type_params () . map (| tp | & tp . ident) . cloned () . collect :: < Vec < _ > > () ; let inject = inject :: resolve_arguments (args . iter () . skip (n) , resolver , & genercs_idents) ; let sign_args = args . iter () . take (n) ; let fixture_args = args . iter () . filter_map (MaybeIdent :: maybe_ident) . map (| arg | parse_quote ! { # arg }) . collect :: < Vec < _ > > () ; let name = Ident :: new (& format ! ("partial_{n}") , Span :: call_site ()) ; let call_get = render_exec_call (parse_quote ! { Self :: get } , & fixture_args , asyncness . is_some () ,) ; quote ! { # [allow (unused_mut)] pub # asyncness fn # name # generics (# (# sign_args) ,*) # output # where_clause { # inject # call_get } } }
};
}
