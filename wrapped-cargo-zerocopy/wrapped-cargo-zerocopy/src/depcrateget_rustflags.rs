// Generated macro for get_rustflags (function)
macro_rules! Depcrateget_rustflags {
() => {
// Module: crate
// Provides: {"get_rustflags"}
// Dependencies: {}
fn get_rustflags (name : & str) -> & 'static str { if name == "nightly" { "--cfg __ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS --cfg zerocopy_derive_union_into_bytes " } else { "--cfg zerocopy_derive_union_into_bytes " } }
};
}
