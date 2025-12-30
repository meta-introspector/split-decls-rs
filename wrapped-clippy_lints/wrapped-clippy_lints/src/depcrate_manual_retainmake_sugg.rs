// Generated macro for make_sugg (function)
macro_rules! Depcrate_manual_retainmake_sugg {
() => {
// Module: crate::manual_retain
// Provides: {"make_sugg"}
// Dependencies: {}
fn make_sugg (cx : & LateContext < '_ > , key_pat : & rustc_hir :: Pat < '_ > , value_pat : & rustc_hir :: Pat < '_ > , left_expr : & hir :: Expr < '_ > , filter_body : & hir :: Body < '_ > ,) -> Option < String > { match (& key_pat . kind , & value_pat . kind) { (hir :: PatKind :: Binding (_ , _ , key_param_ident , None) , hir :: PatKind :: Binding (_ , _ , value_param_ident , None)) => { Some (format ! ("{}.retain(|{key_param_ident}, &mut {value_param_ident}| {})" , snippet (cx , left_expr . span , "..") , snippet (cx , filter_body . value . span , ".."))) } , (hir :: PatKind :: Binding (_ , _ , key_param_ident , None) , hir :: PatKind :: Wild) => Some (format ! ("{}.retain(|{key_param_ident}, _| {})" , snippet (cx , left_expr . span , "..") , snippet (cx , filter_body . value . span , ".."))) , (hir :: PatKind :: Wild , hir :: PatKind :: Binding (_ , _ , value_param_ident , None)) => Some (format ! ("{}.retain(|_, &mut {value_param_ident}| {})" , snippet (cx , left_expr . span , "..") , snippet (cx , filter_body . value . span , ".."))) , _ => None , } }
};
}
