// Generated macro for to_string_pretty (function)
macro_rules! Depcrate_content_jsonto_string_pretty {
() => {
// Module: crate::content::json
// Provides: {"to_string_pretty"}
// Dependencies: {}
# [doc = " Serializes a value to JSON pretty"] # [allow (unused)] pub fn to_string_pretty (value : & Content) -> String { let mut ser = Serializer :: new () ; ser . format = Format :: Pretty ; ser . serialize (value) ; ser . into_result () }
};
}
