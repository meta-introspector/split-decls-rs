// Generated macro for find_good_method_for_matches_macro (function)
macro_rules! Depcrate_matches_redundant_pattern_matchfind_good_method_for_matches_macro {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"find_good_method_for_matches_macro"}
// Dependencies: {}
fn find_good_method_for_matches_macro < 'a , 'tcx > (cx : & LateContext < '_ > , arms : & 'tcx [Arm < 'tcx > ; 2] , path_left : & QPath < '_ > , expected_item_left : Item , should_be_left : & 'a str , should_be_right : & 'a str ,) -> Option < (& 'a str , Option < & 'tcx Expr < 'tcx > >) > { let first_pat = arms [0] . pat ; let body_node_pair = if is_pat_variant (cx , first_pat , path_left , expected_item_left) { (& arms [0] . body . kind , & arms [1] . body . kind) } else { return None ; } ; match body_node_pair { (ExprKind :: Lit (lit_left) , ExprKind :: Lit (lit_right)) => match (& lit_left . node , & lit_right . node) { (LitKind :: Bool (true) , LitKind :: Bool (false)) => Some ((should_be_left , arms [0] . guard)) , (LitKind :: Bool (false) , LitKind :: Bool (true)) => Some ((should_be_right , arms [1] . guard)) , _ => None , } , _ => None , } }
};
}
