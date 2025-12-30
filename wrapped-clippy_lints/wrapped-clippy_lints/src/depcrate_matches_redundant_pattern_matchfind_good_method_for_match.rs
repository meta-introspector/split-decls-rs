// Generated macro for find_good_method_for_match (function)
macro_rules! Depcrate_matches_redundant_pattern_matchfind_good_method_for_match {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"find_good_method_for_match"}
// Dependencies: {}
# [expect (clippy :: too_many_arguments)] fn find_good_method_for_match < 'a , 'tcx > (cx : & LateContext < '_ > , arms : & 'tcx [Arm < 'tcx > ; 2] , path_left : & QPath < '_ > , path_right : & QPath < '_ > , expected_item_left : Item , expected_item_right : Item , should_be_left : & 'a str , should_be_right : & 'a str ,) -> Option < (& 'a str , Option < & 'tcx Expr < 'tcx > >) > { let first_pat = arms [0] . pat ; let second_pat = arms [1] . pat ; let body_node_pair = if (is_pat_variant (cx , first_pat , path_left , expected_item_left)) && (is_pat_variant (cx , second_pat , path_right , expected_item_right)) { (& arms [0] . body . kind , & arms [1] . body . kind) } else if (is_pat_variant (cx , first_pat , path_left , expected_item_right)) && (is_pat_variant (cx , second_pat , path_right , expected_item_left)) { (& arms [1] . body . kind , & arms [0] . body . kind) } else { return None ; } ; match body_node_pair { (ExprKind :: Lit (lit_left) , ExprKind :: Lit (lit_right)) => match (& lit_left . node , & lit_right . node) { (LitKind :: Bool (true) , LitKind :: Bool (false)) => Some ((should_be_left , arms [0] . guard)) , (LitKind :: Bool (false) , LitKind :: Bool (true)) => Some ((should_be_right , arms [1] . guard)) , _ => None , } , _ => None , } }
};
}
