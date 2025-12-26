pub mod diagnostics {
    use rustc_session::parse::ParseSess;
    use rustc_span::{Span, Ident};
    use rustc_ast::tokenstream::TokenStream;
    //    use MacroRule;
    use crate::MacroRule;
    use rustc_errors::ErrorGuaranteed;
    use super::mbe_macro_parser::NamedMatches;
    use rustc_expand_base_lib::resolver_traits::OpaqueDeriveResolution;
    pub enum FailedMacro<'a> {
        Func,
        Attr(&'a TokenStream),
        Derive,
    }
    pub fn failed_to_match_macro<'a, DRT: OpaqueDeriveResolution>(
        _psess: &'a ParseSess,
        _sp: Span,
        _def_span: Span,
        _name: Ident,
        _args: FailedMacro<'_>,
        _body: &TokenStream,
        _rules: &[MacroRule],
    ) -> (Span, ErrorGuaranteed) {
        unimplemented!()
    }
}
