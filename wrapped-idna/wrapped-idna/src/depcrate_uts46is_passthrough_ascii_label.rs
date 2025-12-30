// Generated macro for is_passthrough_ascii_label (function)
macro_rules! Depcrate_uts46is_passthrough_ascii_label {
() => {
// Module: crate::uts46
// Provides: {"is_passthrough_ascii_label"}
// Dependencies: {}
# [inline (always)] fn is_passthrough_ascii_label (label : & [u8]) -> bool { if label . len () >= 4 && label [2] == b'-' && label [3] == b'-' { return false ; } if let Some ((& first , tail)) = label . split_first () { if ! in_inclusive_range8 (first , b'a' , b'z') { return false ; } for & b in tail { if in_inclusive_range8 (b , b'a' , b'z') { continue ; } if in_inclusive_range8 (b , b'0' , b'9') { continue ; } if b == b'-' { continue ; } return false ; } label . last () != Some (& b'-') } else { true } }
};
}
