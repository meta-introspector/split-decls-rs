// Generated macro for deduplicate_responses (function)
macro_rules! Depcrate_export_impldeduplicate_responses {
() => {
// Module: crate::export_impl
// Provides: {"deduplicate_responses"}
// Dependencies: {}
fn deduplicate_responses < 'a > (maximal : bool , marker : DataMarkerInfo , responses : HashMap < DataIdentifierCow < 'a > , (DataResponse < ExportMarker > , Duration) > , fallbacker : & LocaleFallbacker , sink : & dyn DataExporter ,) -> Result < Option < (Duration , DataIdentifierCow < 'a >) > , DataError > { let fallbacker_with_config = fallbacker . for_config (marker . fallback_config) ; responses . iter () . try_for_each (| (id , (response , _duration)) | { if id . locale . is_unknown () { return sink . put_payload (marker , id . as_borrowed () , & response . payload) . map_err (| e | { e . with_req (marker , DataRequest { id : id . as_borrowed () , .. Default :: default () } ,) }) ; } let mut iter = fallbacker_with_config . fallback_for (id . locale) ; loop { if ! maximal { iter . step () ; } if iter . get () . is_unknown () { break ; } if maximal { iter . step () ; } if let Some ((inherited_response , _duration)) = responses . get (& DataIdentifierBorrowed :: for_marker_attributes_and_locale (& id . marker_attributes , iter . get () ,) . as_cow () ,) { if inherited_response . payload == response . payload { log :: trace ! ("Deduplicating {id} (inherits from {})" , iter . get ()) ; return Ok (()) ; } else { break ; } } } sink . put_payload (marker , id . as_borrowed () , & response . payload) . map_err (| e | { e . with_req (marker , DataRequest { id : id . as_borrowed () , .. Default :: default () } ,) }) }) ? ; Ok (responses . into_iter () . map (| (id , (_response , duration)) | (duration , id)) . max ()) }
};
}
