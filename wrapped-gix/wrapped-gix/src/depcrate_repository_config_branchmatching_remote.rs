// Generated macro for matching_remote (function)
macro_rules! Depcrate_repository_config_branchmatching_remote {
() => {
// Module: crate::repository::config::branch
// Provides: {"matching_remote"}
// Dependencies: {}
fn matching_remote < 'a > (lhs : & FullNameRef , specs : impl IntoIterator < Item = & 'a gix_refspec :: RefSpec > , object_hash : gix_hash :: Kind ,) -> Option < Result < Cow < 'static , FullNameRef > , gix_validate :: reference :: name :: Error > > { let search = gix_refspec :: MatchGroup { specs : specs . into_iter () . map (gix_refspec :: RefSpec :: to_ref) . filter (| spec | spec . source () . is_some () && spec . destination () . is_some ()) . collect () , } ; let null_id = object_hash . null () ; let out = search . match_lhs (Some (gix_refspec :: match_group :: Item { full_ref_name : lhs . as_bstr () , target : & null_id , object : None , }) . into_iter () ,) ; out . mappings . into_iter () . next () . and_then (| m | m . rhs . map (| name | FullName :: try_from (name . into_owned ()) . map (Cow :: Owned))) }
};
}
