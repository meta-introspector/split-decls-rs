// Generated macro for write_conflict_marker (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilswrite_conflict_marker {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"write_conflict_marker"}
// Dependencies: {}
pub fn write_conflict_marker (out : & mut Vec < u8 > , marker : u8 , label : Option < & BStr > , marker_size : u8 , nl : & BStr) { assure_ends_with_nl (out , nl) ; out . extend (std :: iter :: repeat_n (marker , marker_size as usize)) ; if let Some (label) = label { out . push (b' ') ; out . extend_from_slice (label) ; } out . push_str (nl) ; }
};
}
