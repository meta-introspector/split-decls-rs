// Generated macro for try_parse_pattern (function)
macro_rules! Depcrate_matches_manual_utilstry_parse_pattern {
() => {
// Module: crate::matches::manual_utils
// Provides: {"try_parse_pattern"}
// Dependencies: {}
pub (super) fn try_parse_pattern < 'tcx > (cx : & LateContext < 'tcx > , pat : & 'tcx Pat < '_ > , ctxt : SyntaxContext ,) -> Option < OptionPat < 'tcx > > { fn f < 'tcx > (cx : & LateContext < 'tcx > , pat : & 'tcx Pat < '_ > , ref_count : usize , ctxt : SyntaxContext ,) -> Option < OptionPat < 'tcx > > { match pat . kind { PatKind :: Wild => Some (OptionPat :: Wild) , PatKind :: Ref (pat , _ , _) => f (cx , pat , ref_count + 1 , ctxt) , _ if is_none_pattern (cx , pat) => Some (OptionPat :: None) , _ if let Some ([pattern]) = as_some_pattern (cx , pat) && pat . span . ctxt () == ctxt => { Some (OptionPat :: Some { pattern , ref_count }) } , _ => None , } } f (cx , pat , 0 , ctxt) }
};
}
