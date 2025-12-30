// Generated macro for consume_option_as_ref (function)
macro_rules! Depcrate_unwrapconsume_option_as_ref {
() => {
// Module: crate::unwrap
// Provides: {"consume_option_as_ref"}
// Dependencies: {}
# [doc = " Checks if the expression is a method call to `as_{ref,mut}` and returns the receiver of it."] # [doc = " If it isn't, the expression itself is returned."] fn consume_option_as_ref < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> (& 'tcx Expr < 'tcx > , Option < AsRefKind >) { if let ExprKind :: MethodCall (path , recv , [] , _) = expr . kind { match path . ident . name { sym :: as_ref => (recv , Some (AsRefKind :: AsRef)) , sym :: as_mut => (recv , Some (AsRefKind :: AsMut)) , _ => (expr , None) , } } else { (expr , None) } }
};
}
