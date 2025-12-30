// Generated macro for to_string (function)
macro_rules! Depcrate_content_jsonto_string {
() => {
// Module: crate::content::json
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Serializes a value to JSON."] pub fn to_string (value : & Content) -> String { let mut ser = Serializer :: new () ; ser . serialize (value) ; ser . into_result () }
};
}
