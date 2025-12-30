// Generated macro for for_each_expr (function)
macro_rules! Depcrate_visitorsfor_each_expr {
() => {
// Module: crate::visitors
// Provides: {"for_each_expr"}
// Dependencies: {}
# [doc = " Calls the given function once for each expression contained. This will enter bodies, but not"] # [doc = " nested items."] pub fn for_each_expr < 'tcx , B , C : Continue > (cx : & LateContext < 'tcx > , node : impl Visitable < 'tcx > , f : impl FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B , C > ,) -> Option < B > { struct V < 'tcx , F > { tcx : TyCtxt < 'tcx > , f : F , } impl < 'tcx , B , C : Continue , F : FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B , C > > Visitor < 'tcx > for V < 'tcx , F > { type NestedFilter = nested_filter :: OnlyBodies ; type Result = ControlFlow < B > ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_expr (& mut self , e : & 'tcx Expr < 'tcx >) -> Self :: Result { match (self . f) (e) { ControlFlow :: Continue (c) if c . descend () => walk_expr (self , e) , ControlFlow :: Break (b) => ControlFlow :: Break (b) , ControlFlow :: Continue (_) => ControlFlow :: Continue (()) , } } fn visit_anon_const (& mut self , _ : & 'tcx AnonConst) -> Self :: Result { ControlFlow :: Continue (()) } fn visit_ty (& mut self , _ : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> Self :: Result { ControlFlow :: Continue (()) } fn visit_pat (& mut self , _ : & 'tcx Pat < 'tcx >) -> Self :: Result { ControlFlow :: Continue (()) } fn visit_qpath (& mut self , _ : & 'tcx QPath < 'tcx > , _ : HirId , _ : Span) -> Self :: Result { ControlFlow :: Continue (()) } fn visit_nested_item (& mut self , _ : ItemId) -> Self :: Result { ControlFlow :: Continue (()) } } let mut v = V { tcx : cx . tcx , f } ; node . visit (& mut v) . break_value () }
};
}
