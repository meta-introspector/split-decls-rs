// Generated macro for tests (module)
macro_rules! Depcrate_meta_regextests {
() => {
// Module: crate::meta::regex
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn regression_suffix_literal_count () { let _ = env_logger :: try_init () ; let re = Regex :: new (r"[a-zA-Z]+ing") . unwrap () ; assert_eq ! (1 , re . find_iter ("tingling") . count ()) ; } }
};
}
