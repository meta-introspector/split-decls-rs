// Generated macro for is_wildcard_arm (function)
macro_rules! Depcrate_matches_match_same_armsis_wildcard_arm {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"is_wildcard_arm"}
// Dependencies: {}
fn is_wildcard_arm (pat : & Pat < '_ >) -> bool { match pat . kind { PatKind :: Wild => true , PatKind :: Or ([.. , last]) => matches ! (last . kind , PatKind :: Wild) , _ => false , } }
};
}
