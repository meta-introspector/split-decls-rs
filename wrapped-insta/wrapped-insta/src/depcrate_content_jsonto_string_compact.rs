// Generated macro for to_string_compact (function)
macro_rules! Depcrate_content_jsonto_string_compact {
() => {
// Module: crate::content::json
// Provides: {"to_string_compact"}
// Dependencies: {}
# [doc = " Serializes a value to JSON in single-line format."] # [allow (unused)] pub fn to_string_compact (value : & Content) -> String { let mut ser = Serializer :: new () ; ser . format = Format :: SingleLine ; ser . serialize (value) ; let rv = ser . into_result () ; if rv . chars () . count () > COMPACT_MAX_CHARS { to_string_pretty (value) } else { rv } }
};
}
