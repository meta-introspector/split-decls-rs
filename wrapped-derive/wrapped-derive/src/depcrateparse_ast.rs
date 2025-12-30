// Generated macro for parse_ast (function)
macro_rules! Depcrateparse_ast {
() => {
// Module: crate
// Provides: {"parse_ast"}
// Dependencies: {}
fn parse_ast (ast : DeriveInput) -> Result < TlsStruct > { let call_site = Span :: call_site () ; let ident = ast . ident . clone () ; let generics = ast . generics . clone () ; match ast . data { Data :: Struct (st) => { let members = fields_to_members (& st . fields) ; let member_prefixes = fields_to_member_prefixes (& st . fields) ? ; let member_skips = fields_to_member_skips (& st . fields) ? ; Ok (TlsStruct :: Struct (Struct { call_site , ident , generics , members , member_prefixes , member_skips , })) } Data :: Enum (syn :: DataEnum { variants , .. }) => { let mut repr = None ; for attr in ast . attrs { if attr . path () . is_ident ("repr") { let ty = attr . parse_args () ? ; repr = Some (ty) ; break ; } } let repr = repr . ok_or_else (| | syn :: Error :: new (call_site , "missing #[repr(...)] attribute")) ? ; let discriminant_constants = define_discriminant_constants (& ident , & repr , & variants) ? ; let variants = variants . into_iter () . map (| variant | { Ok (Variant { ident : variant . ident , members : fields_to_members (& variant . fields) , member_prefixes : fields_to_member_prefixes (& variant . fields) ? , }) }) . collect :: < Result < Vec < _ > > > () ? ; Ok (TlsStruct :: Enum (Enum { call_site , ident , generics , repr , variants , discriminant_constants , })) } Data :: Union (_) => unimplemented ! () , } }
};
}
