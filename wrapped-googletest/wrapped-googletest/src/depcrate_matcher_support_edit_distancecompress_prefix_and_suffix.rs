// Generated macro for compress_prefix_and_suffix (function)
macro_rules! Depcrate_matcher_support_edit_distancecompress_prefix_and_suffix {
() => {
// Module: crate::matcher_support::edit_distance
// Provides: {"compress_prefix_and_suffix"}
// Dependencies: {}
fn compress_prefix_and_suffix < T > (edits : & mut Vec < Edit < T > >) { if let Some (mut first_non_extra_actual_edit) = edits . iter () . position (| e | ! matches ! (e , Edit :: ExtraActual (_))) { if first_non_extra_actual_edit > 1 && matches ! (edits [first_non_extra_actual_edit] , Edit :: ExtraExpected (_)) { first_non_extra_actual_edit -= 1 ; } edits . splice (.. first_non_extra_actual_edit , [Edit :: AdditionalActual]) ; } if let Some (mut last_non_extra_actual_edit) = edits . iter () . rposition (| e | ! matches ! (e , Edit :: ExtraActual (_))) { if last_non_extra_actual_edit < edits . len () - 1 && matches ! (edits [last_non_extra_actual_edit] , Edit :: ExtraExpected (_)) { last_non_extra_actual_edit += 1 ; } edits . splice (last_non_extra_actual_edit + 1 .. , [Edit :: AdditionalActual]) ; } }
};
}
