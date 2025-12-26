pub mod mbe {
    pub mod transcribe {
        use rustc_errors::PResult;
        use rustc_ast::tokenstream::TokenStream;
        use rustc_session::parse::ParseSess;
        use rustc_data_structures::fx::FxHashMap;
        use rustc_span::hygiene::Transparency;
        use rustc_span::hygiene::LocalExpnId;
        use super::super::mbe_macro_parser::NamedMatches;
        use lib_token_tree::Delimited;
        use rustc_span::MacroRulesNormalizedIdent;


        pub fn transcribe<'a>(
            _psess: &'a ParseSess,
            _interp: &'a FxHashMap<MacroRulesNormalizedIdent, NamedMatches>,
            _src: &'a Delimited,
            _src_span: rustc_ast::tokenstream::DelimSpan,
            _transparency: Transparency,
            _expand_id: LocalExpnId,
        ) -> PResult<'a, TokenStream> {
            unimplemented!()
        }
    }
}
