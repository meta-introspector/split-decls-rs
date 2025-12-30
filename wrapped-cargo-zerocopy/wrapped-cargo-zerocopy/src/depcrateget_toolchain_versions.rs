// Generated macro for get_toolchain_versions (function)
macro_rules! Depcrateget_toolchain_versions {
() => {
// Module: crate
// Provides: {"get_toolchain_versions"}
// Dependencies: {}
fn get_toolchain_versions () -> Versions { let manifest_text = fs :: read_to_string ("Cargo.toml") . unwrap () ; let manifest = toml :: from_str :: < Value > (& manifest_text) . unwrap () ; let package = manifest . as_table () . unwrap () ["package"] . as_table () . unwrap () ; let metadata = package ["metadata"] . as_table () . unwrap () ; let build_rs = metadata ["build-rs"] . as_table () . unwrap () ; let ci = metadata ["ci"] . as_table () . unwrap () ; Versions { msrv : package ["rust-version"] . as_str () . unwrap () . to_string () , stable : ci ["pinned-stable"] . as_str () . unwrap () . to_string () , nightly : ci ["pinned-nightly"] . as_str () . unwrap () . to_string () , build_rs : build_rs . clone () , } }
};
}
