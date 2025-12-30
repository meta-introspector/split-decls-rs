// Generated macro for impl_42 (impl)
macro_rules! Depcrate_common_diagnosticimpl_42 {
() => {
// Module: crate::common::diagnostic
// Provides: {"impl_42"}
// Dependencies: {}
impl Scope { fn spec_link (& self) -> String { format ! ("{SPEC_URL}{}" , self . spec_section ()) } pub (crate) fn custom < S : AsRef < str > > (& self , span : Span , msg : S) -> Diagnostic { Diagnostic :: spanned (span , format ! ("{self} {}" , msg . as_ref ())) . note (self . spec_link ()) } pub (crate) fn error (& self , err : & syn :: Error) -> Diagnostic { Diagnostic :: spanned (err . span () , format ! ("{self} {err}")) . note (self . spec_link ()) } pub (crate) fn emit_custom < S : AsRef < str > > (& self , span : Span , msg : S) { self . custom (span , msg) . emit () } pub (crate) fn custom_error < S : AsRef < str > > (& self , span : Span , msg : S) -> syn :: Error { syn :: Error :: new (span , format ! ("{self} {}" , msg . as_ref ())) } pub (crate) fn no_double_underscore (& self , field : Span) { Diagnostic :: spanned (field , "All types and directives defined within a schema must not have a name which begins \
             with `__` (two underscores), as this is used exclusively by GraphQL’s introspection \
             system." ,) . note (format ! ("{SPEC_URL}#sec-Schema")) . emit () ; } }
};
}
