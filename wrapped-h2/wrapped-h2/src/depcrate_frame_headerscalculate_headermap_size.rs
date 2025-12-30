// Generated macro for calculate_headermap_size (function)
macro_rules! Depcrate_frame_headerscalculate_headermap_size {
() => {
// Module: crate::frame::headers
// Provides: {"calculate_headermap_size"}
// Dependencies: {}
fn calculate_headermap_size (map : & HeaderMap) -> usize { map . iter () . map (| (name , value) | decoded_header_size (name . as_str () . len () , value . len ())) . sum :: < usize > () }
};
}
