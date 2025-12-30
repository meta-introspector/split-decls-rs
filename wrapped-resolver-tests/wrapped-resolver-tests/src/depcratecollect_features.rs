// Generated macro for collect_features (function)
macro_rules! Depcratecollect_features {
() => {
// Module: crate
// Provides: {"collect_features"}
// Dependencies: {}
fn collect_features (resolve : & Resolve) -> Vec < (PackageId , Vec < InternedString >) > { resolve . sort () . iter () . map (| & pkg | (pkg , resolve . features (pkg) . to_vec ())) . collect () }
};
}
