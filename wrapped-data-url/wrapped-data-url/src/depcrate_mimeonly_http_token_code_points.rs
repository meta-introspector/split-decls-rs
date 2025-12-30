// Generated macro for only_http_token_code_points (function)
macro_rules! Depcrate_mimeonly_http_token_code_points {
() => {
// Module: crate::mime
// Provides: {"only_http_token_code_points"}
// Dependencies: {}
fn only_http_token_code_points (s : & str) -> bool { s . bytes () . all (| byte | IS_HTTP_TOKEN [byte as usize]) }
};
}
