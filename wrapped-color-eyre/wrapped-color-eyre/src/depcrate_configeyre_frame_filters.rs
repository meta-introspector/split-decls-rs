// Generated macro for eyre_frame_filters (function)
macro_rules! Depcrate_configeyre_frame_filters {
() => {
// Module: crate::config
// Provides: {"eyre_frame_filters"}
// Dependencies: {}
fn eyre_frame_filters (frames : & mut Vec < & Frame >) { let filters = & ["<color_eyre::Handler as eyre::EyreHandler>::default" , "eyre::" , "color_eyre::" ,] ; frames . retain (| frame | { ! filters . iter () . any (| f | { let name = if let Some (name) = frame . name . as_ref () { name . as_str () } else { return true ; } ; name . starts_with (f) }) }) ; }
};
}
