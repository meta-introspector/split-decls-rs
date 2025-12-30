// Generated macro for impl_93 (impl)
macro_rules! Depcrate_mac_resultimpl_93 {
() => {
// Module: crate::mac_result
// Provides: {"impl_93"}
// Dependencies: {}
impl < DRT : OpaqueDeriveResolution + 'static > MacResult < DRT > for MacEager { fn make_expr (self : Box < Self >) -> Option < Box < ast :: Expr > > { self . expr } fn make_items (self : Box < Self >) -> Option < SmallVec < Box < ast :: Item > , 1 > > { self . items } fn make_impl_items (self : Box < Self >) -> Option < SmallVec < Box < ast :: AssocItem > , 1 > > { self . impl_items } fn make_trait_impl_items (self : Box < Self >) -> Option < SmallVec < Box < ast :: AssocItem > , 1 > > { self . impl_items } fn make_trait_items (self : Box < Self >) -> Option < SmallVec < Box < ast :: AssocItem > , 1 > > { self . trait_items } fn make_foreign_items (self : Box < Self >) -> Option < SmallVec < Box < ast :: ForeignItem > , 1 > > { self . foreign_items } fn make_stmts (self : Box < Self >) -> Option < SmallVec < ast :: Stmt , 1 > > { match self . stmts . as_ref () . map_or (0 , | s | s . len ()) { 0 => make_stmts_default ! (self , DRT) , _ => self . stmts , } } fn make_pat (self : Box < Self >) -> Option < Box < ast :: Pat > > { if let Some (p) = self . pat { return Some (p) ; } if let Some (e) = self . expr { if matches ! (e . kind , ExprKind :: Lit (_) | ExprKind :: IncludedBytes (_)) { return Some (Box :: new (ast :: Pat { id : ast :: DUMMY_NODE_ID , span : e . span , kind : PatKind :: Expr (e) , tokens : None , })) ; } } None } fn make_ty (self : Box < Self >) -> Option < Box < ast :: Ty > > { self . ty } }
};
}
