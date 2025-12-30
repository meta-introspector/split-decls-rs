// Generated macro for extract_package_name (function)
macro_rules! Depcrateextract_package_name {
() => {
// Module: crate
// Provides: {"extract_package_name"}
// Dependencies: {}
fn extract_package_name (cargo_toml : & DocumentMut) -> Option < & str > { cargo_toml . get ("package") ? . get ("name") ? . as_str () }
};
}
