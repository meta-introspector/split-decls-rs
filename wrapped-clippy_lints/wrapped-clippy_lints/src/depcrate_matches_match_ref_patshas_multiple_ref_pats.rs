// Generated macro for has_multiple_ref_pats (function)
macro_rules! Depcrate_matches_match_ref_patshas_multiple_ref_pats {
() => {
// Module: crate::matches::match_ref_pats
// Provides: {"has_multiple_ref_pats"}
// Dependencies: {}
fn has_multiple_ref_pats < 'a , 'b , I > (pats : I) -> bool where 'b : 'a , I : Iterator < Item = & 'a Pat < 'b > > , { let mut ref_count = 0 ; for opt in pats . map (| pat | match pat . kind { PatKind :: Ref (..) => Some (true) , PatKind :: Wild => Some (false) , _ => None , }) { if let Some (inner) = opt { if inner { ref_count += 1 ; } } else { return false ; } } ref_count > 1 }
};
}
