// Generated macro for get_good_method (function)
macro_rules! Depcrate_matches_redundant_pattern_matchget_good_method {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"get_good_method"}
// Dependencies: {}
fn get_good_method < 'tcx > (cx : & LateContext < '_ > , arms : & 'tcx [Arm < 'tcx > ; 2] , path_left : & QPath < '_ > ,) -> Option < (& 'static str , Option < & 'tcx Expr < 'tcx > >) > { let ident = get_ident (path_left) ? ; let (expected_item_left , should_be_left , should_be_right) = match ident . name { sym :: Ok => (Item :: Lang (ResultOk) , "is_ok()" , "is_err()") , sym :: Err => (Item :: Lang (ResultErr) , "is_err()" , "is_ok()") , sym :: Some => (Item :: Lang (OptionSome) , "is_some()" , "is_none()") , sym :: None => (Item :: Lang (OptionNone) , "is_none()" , "is_some()") , sym :: Ready => (Item :: Lang (PollReady) , "is_ready()" , "is_pending()") , sym :: Pending => (Item :: Lang (PollPending) , "is_pending()" , "is_ready()") , sym :: V4 => (Item :: Diag (sym :: IpAddr , sym :: V4) , "is_ipv4()" , "is_ipv6()") , sym :: V6 => (Item :: Diag (sym :: IpAddr , sym :: V6) , "is_ipv6()" , "is_ipv4()") , _ => return None , } ; find_good_method_for_matches_macro (cx , arms , path_left , expected_item_left , should_be_left , should_be_right) }
};
}
