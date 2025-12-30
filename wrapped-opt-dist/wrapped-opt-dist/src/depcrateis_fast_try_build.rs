// Generated macro for is_fast_try_build (function)
macro_rules! Depcrateis_fast_try_build {
() => {
// Module: crate
// Provides: {"is_fast_try_build"}
// Dependencies: {}
# [doc = " For a fast try build, we want to only build the bare minimum of components to get a"] # [doc = " working toolchain, and not run any tests."] fn is_fast_try_build () -> bool { std :: env :: var ("DIST_TRY_BUILD") . unwrap_or_else (| _ | "0" . to_string ()) != "0" }
};
}
