pub mod macro_rules_utils {
    use rustc_ast::tokenstream::TokenStream;
    use rustc_errors::DiagCtxtHandle;
    use rustc_session::parse::ParseSess;
    use rustc_span::{Span, Ident};
    use rustc_parse::parser::{Parser, Recovery};
    use rustc_data_structures::fx::FxIndexMap;

    pub fn parser_from_cx<'a>(psess: &'a ParseSess, tt: TokenStream, recovery: Recovery) -> Parser<'a> {
        unimplemented!()
    }

    pub fn trace_macros_note_expander(cx_expansions: &mut FxIndexMap<Span, Vec<String>>, sp: Span, message: String) {
        unimplemented!()
    }

    pub fn is_defined_in_current_crate(_node_id: rustc_ast::NodeId) -> bool {
        unimplemented!()
    }
    pub fn has_compile_error_macro(_s: &ParseSess) -> bool { unimplemented!() }
}

