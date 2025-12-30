// Generated macro for is_non_value_js_keyword (function)
macro_rules! Depcrate_parseris_non_value_js_keyword {
() => {
// Module: crate::parser
// Provides: {"is_non_value_js_keyword"}
// Dependencies: {}
# [doc = " Returns whether the given string is a JS keyword that does NOT behave like"] # [doc = " a value."] # [doc = ""] # [doc = " Value-like keywords can be called like functions or have properties"] # [doc = " accessed, which makes it possible to use them in imports. In general,"] # [doc = " imports should use this function to check for reserved keywords."] fn is_non_value_js_keyword (keyword : & str) -> bool { JS_KEYWORDS . contains (& keyword) && ! VALUE_LIKE_JS_KEYWORDS . contains (& keyword) }
};
}
