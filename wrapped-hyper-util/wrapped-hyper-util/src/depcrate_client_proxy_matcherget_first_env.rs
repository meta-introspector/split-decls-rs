// Generated macro for get_first_env (function)
macro_rules! Depcrate_client_proxy_matcherget_first_env {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"get_first_env"}
// Dependencies: {}
fn get_first_env (names : & [& str]) -> String { for name in names { if let Ok (val) = std :: env :: var (name) { return val ; } } String :: new () }
};
}
