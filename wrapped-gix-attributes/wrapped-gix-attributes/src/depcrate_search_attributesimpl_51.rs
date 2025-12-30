// Generated macro for impl_51 (impl)
macro_rules! Depcrate_search_attributesimpl_51 {
() => {
// Module: crate::search::attributes
// Provides: {"impl_51"}
// Dependencies: {}
impl Pattern for Attributes { type Value = Value ; fn bytes_to_patterns (& self , bytes : & [u8] , _source : & std :: path :: Path) -> Vec < pattern :: Mapping < Self :: Value > > { fn into_owned_assignments < 'a > (attrs : impl Iterator < Item = Result < crate :: AssignmentRef < 'a > , crate :: name :: Error > > ,) -> Option < Assignments > { let res = attrs . map (| res | { res . map (| a | TrackedAssignment { id : Default :: default () , inner : a . to_owned () , }) }) . collect :: < Result < Assignments , _ > > () ; match res { Ok (res) => Some (res) , Err (_err) => { gix_trace :: warn ! ("{}" , _err) ; None } } } crate :: parse (bytes) . filter_map (| res | match res { Ok (pattern) => Some (pattern) , Err (_err) => { gix_trace :: warn ! ("{}: {}" , _source . display () , _err) ; None } }) . filter_map (| (pattern_kind , assignments , line_number) | { let (pattern , value) = match pattern_kind { crate :: parse :: Kind :: Macro (macro_name) => (gix_glob :: Pattern { text : macro_name . as_str () . into () , mode : macro_mode () , first_wildcard_pos : None , } , Value :: MacroAssignments { id : Default :: default () , assignments : into_owned_assignments (assignments) ? , } ,) , crate :: parse :: Kind :: Pattern (p) => ((! p . is_negative ()) . then_some (p) ? , Value :: Assignments (into_owned_assignments (assignments) ?) ,) , } ; pattern :: Mapping { pattern , value , sequence_number : line_number , } . into () }) . collect () } }
};
}
