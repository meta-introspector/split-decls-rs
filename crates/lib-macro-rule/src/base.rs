pub mod base {
    use rustc_errors::ErrorGuaranteed;
    use rustc_ast::tokenstream::TokenStream;
    use rustc_span::Span;
    use rustc_expand_base_lib::prelude::OpaqueDeriveResolution;
    use rustc_ast::NodeId;
    use rustc_span::Ident;
    use rustc_session::parse::ParseSess;
    use rustc_span::hygiene::Transparency;
//    use MacroRule;
    use super::mbe_macro_parser::NamedMatches;
    use rustc_data_structures::fx::FxHashMap;
    use lib_token_tree::Delimited;
    use rustc_ast::BlockCheckMode;
    use rustc_ast::Expr;
    use rustc_ast::Pat;
    use rustc_ast::Item;
//    use rustc_ast::ast::ImplItem;
//    use rustc_ast::ast::TraitItem;
    use rustc_ast::Ty;
    use rustc_ast::Stmt;
    use rustc_ast::Block;
    use smallvec::SmallVec;
    use rustc_span::hygiene::ExpnData;
    use rustc_errors::DiagCtxtHandle;


    pub trait MacResult<DRT: OpaqueDeriveResolution + 'static, ImplItem, TraitItem> {
        fn make_expr(self: Box<Self>) -> Option<Box<Expr>> { unimplemented!() }
        fn make_pat(self: Box<Self>) -> Option<Box<Pat>> { unimplemented!() }
        fn make_items(self: Box<Self>) -> Option<SmallVec<Box<Item>, 1>> { unimplemented!() }
        fn make_impl_items(self: Box<Self>) -> Option<SmallVec<[ImplItem; 1]>> { unimplemented!() }
        fn make_trait_items(self: Box<Self>) -> Option<SmallVec<[TraitItem; 1]>> { unimplemented!() }
        fn make_type(self: Box<Self>) -> Option<Box<Ty>> { unimplemented!() }
        fn make_stmts(self: Box<Self>) -> Option<SmallVec<[Stmt; 1]>> { unimplemented!() }
        fn make_block(self: Box<Self>) -> Option<Block> { unimplemented!() }
        //fn make_ast_fragment(self: Box<Self>) -> Option<rustc_expand::expand::AstFragment> { unimplemented!() }

    }


    pub struct DummyResult(Option<ErrorGuaranteed>);

    impl<DRT: OpaqueDeriveResolution + 'static, ImplItem, TraitItem> MacResult<DRT, ImplItem, TraitItem> for DummyResult {
        fn make_expr(self: Box<Self>) -> Option<Box<Expr>> { self.0.map_or(None, |guar| Some(Box::new(Expr { /* ... */ span: Span::call_site(), kind: rustc_ast::ExprKind::Err, id: NodeId::PLACEHOLDER, attrs: SmallVec::new() }))) }
        fn make_pat(self: Box<Self>) -> Option<Box<Pat>> { self.0.map_or(None, |guar| Some(Box::new(Pat { /* ... */ span: Span::call_site(), kind: rustc_ast::PatKind::Wild, id: NodeId::PLACEHOLDER }))) }
        fn make_items(self: Box<Self>) -> Option<SmallVec<[Box<Item>; 1]>> { self.0.map_or(None, |guar| Some(SmallVec::new())) }
        fn make_impl_items(self: Box<Self>) -> Option<SmallVec<[ImplItem; 1]>> { self.0.map_or(None, |guar| Some(SmallVec::new())) }
        fn make_trait_items(self: Box<Self>) -> Option<SmallVec<[TraitItem; 1]>> { self.0.map_or(None, |guar| Some(SmallVec::new())) }
        fn make_type(self: Box<Self>) -> Option<Box<Ty>> { self.0.map_or(None, |guar| Some(Box::new(Ty { /* ... */ span: Span::call_site(), kind: rustc_ast::TyKind::Err, id: NodeId::PLACEHOLDER }))) }
        fn make_stmts(self: Box<Self>) -> Option<SmallVec<[Stmt; 1]>> { self.0.map_or(None, |guar| Some(SmallVec::new())) }
        fn make_block(self: Box<Self>) -> Option<Block> { self.0.map_or(None, |guar| Some(Block { stmts: SmallVec::new(), id: NodeId::PLACEHOLDER, span: Span::call_site(), rules: BlockCheckMode::Default })) }
        //fn make_ast_fragment(self: Box<Self>) -> Option<rustc_expand::expand::AstFragment> { self.0.map_or(None, |guar| Some(rustc_expand::expand::AstFragment::Expr(guar.into()))) }
    }
    impl DummyResult {
        pub fn any<DRT: OpaqueDeriveResolution + 'static>(_sp: Span, guar: ErrorGuaranteed) -> Box<dyn MacResult<DRT> + 'static> { Box::new(DummyResult(Some(guar))) }
    }

    pub struct ExtCtxt<'a, DRT: OpaqueDeriveResolution + 'static> {
        pub sess: &'a ParseSess,
        pub current_expansion: ExpnData,
        pub expansions: FxHashMap<Span, Vec<String>>,
    }

    impl<'a, DRT: OpaqueDeriveResolution + 'static> ExtCtxt<'a, DRT> {
        pub fn psess(&self) -> &ParseSess { self.sess }
        pub fn dcx(&self) -> DiagCtxtHandle { self.sess.dcx() }
        pub fn trace_macros(&self) -> bool { false } // Placeholder
        pub fn macro_error_and_trace_macros_diag(&self) { /* Placeholder */ }
        pub fn trace_macros_diag(&self) { /* Placeholder */ }
    }
}
