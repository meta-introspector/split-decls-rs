// Generated macro for expand_variables (function)
macro_rules! Depcrate_directivesexpand_variables {
() => {
// Module: crate::directives
// Provides: {"expand_variables"}
// Dependencies: {}
fn expand_variables (mut value : String , config : & Config) -> String { const CWD : & str = "{{cwd}}" ; const SRC_BASE : & str = "{{src-base}}" ; const TEST_SUITE_BUILD_BASE : & str = "{{build-base}}" ; const RUST_SRC_BASE : & str = "{{rust-src-base}}" ; const SYSROOT_BASE : & str = "{{sysroot-base}}" ; const TARGET_LINKER : & str = "{{target-linker}}" ; const TARGET : & str = "{{target}}" ; if value . contains (CWD) { let cwd = env :: current_dir () . unwrap () ; value = value . replace (CWD , & cwd . to_str () . unwrap ()) ; } if value . contains (SRC_BASE) { value = value . replace (SRC_BASE , & config . src_test_suite_root . as_str ()) ; } if value . contains (TEST_SUITE_BUILD_BASE) { value = value . replace (TEST_SUITE_BUILD_BASE , & config . build_test_suite_root . as_str ()) ; } if value . contains (SYSROOT_BASE) { value = value . replace (SYSROOT_BASE , & config . sysroot_base . as_str ()) ; } if value . contains (TARGET_LINKER) { value = value . replace (TARGET_LINKER , config . target_linker . as_deref () . unwrap_or ("")) ; } if value . contains (TARGET) { value = value . replace (TARGET , & config . target) ; } if value . contains (RUST_SRC_BASE) { let src_base = config . sysroot_base . join ("lib/rustlib/src/rust") ; src_base . try_exists () . expect (& * format ! ("{} should exists" , src_base)) ; let src_base = src_base . read_link_utf8 () . unwrap_or (src_base) ; value = value . replace (RUST_SRC_BASE , & src_base . as_str ()) ; } value }
};
}
