// Generated macro for find_custom_refname (function)
macro_rules! Depcrate_clone_fetch_utilfind_custom_refname {
() => {
// Module: crate::clone::fetch::util
// Provides: {"find_custom_refname"}
// Dependencies: {}
pub (super) fn find_custom_refname < 'a > (ref_map : & 'a crate :: remote :: fetch :: RefMap , ref_name : & PartialName ,) -> Result < (Option < & 'a gix_hash :: oid > , Option < & 'a BStr >) , Error > { let group = gix_refspec :: MatchGroup :: from_fetch_specs (Some (gix_refspec :: parse (ref_name . as_ref () . as_bstr () , gix_refspec :: parse :: Operation :: Fetch) . expect ("partial names are valid refs") ,)) ; let filtered_items : Vec < _ > = ref_map . mappings . iter () . filter_map (| m | { m . remote . as_name () . and_then (| name | m . remote . as_id () . map (| id | (name , id))) }) . map (| (full_ref_name , target) | gix_refspec :: match_group :: Item { full_ref_name , target , object : None , }) . collect () ; let res = group . match_lhs (filtered_items . iter () . copied ()) ; match res . mappings . len () { 0 => Err (Error :: RefNameMissing { wanted : ref_name . clone () , }) , 1 => { let item = filtered_items [res . mappings [0] . item_index . expect ("we map by name only and have no object-id in refspec")] ; Ok ((Some (item . target) , Some (item . full_ref_name))) } _ => Err (Error :: RefNameAmbiguous { wanted : ref_name . clone () , candidates : res . mappings . into_iter () . filter_map (| m | match m . lhs { gix_refspec :: match_group :: SourceRef :: FullName (name) => Some (name . into_owned ()) , gix_refspec :: match_group :: SourceRef :: ObjectId (_) => None , }) . collect () , }) , } }
};
}
