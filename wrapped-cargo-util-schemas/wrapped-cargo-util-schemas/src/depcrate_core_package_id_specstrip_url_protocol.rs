// Generated macro for strip_url_protocol (function)
macro_rules! Depcrate_core_package_id_specstrip_url_protocol {
() => {
// Module: crate::core::package_id_spec
// Provides: {"strip_url_protocol"}
// Dependencies: {}
fn strip_url_protocol (url : & Url) -> Url { let raw = url . to_string () ; raw . split_once ('+') . unwrap () . 1 . parse () . unwrap () }
};
}
