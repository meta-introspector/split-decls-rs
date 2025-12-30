// Generated macro for fips (module)
macro_rules! Depcratefips {
() => {
// Module: crate
// Provides: {"fips"}
// Dependencies: {}
# [cfg (not (any (libressl , ossl300)))] pub mod fips ;
};
}
