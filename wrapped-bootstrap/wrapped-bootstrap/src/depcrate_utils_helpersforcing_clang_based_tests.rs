// Generated macro for forcing_clang_based_tests (function)
macro_rules! Depcrate_utils_helpersforcing_clang_based_tests {
() => {
// Module: crate::utils::helpers
// Provides: {"forcing_clang_based_tests"}
// Dependencies: {}
pub fn forcing_clang_based_tests () -> bool { if let Some (var) = env :: var_os ("RUSTBUILD_FORCE_CLANG_BASED_TESTS") { match & var . to_string_lossy () . to_lowercase () [..] { "1" | "yes" | "on" => true , "0" | "no" | "off" => false , other => { panic ! ("Unrecognized option '{other}' set in \
                        RUSTBUILD_FORCE_CLANG_BASED_TESTS") } } } else { false } }
};
}
