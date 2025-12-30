// Generated macro for tests (module)
macro_rules! Depcrate_typestests {
() => {
// Module: crate::types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: string :: ToString ; use super :: * ; # [test] fn encoding () { # [allow (unexpected_cfgs)] let expected = if cfg ! (target_env = "sim") { "{MTLResourceID=(?=QQ)}" } else { "{MTLResourceID=Q}" } ; assert_eq ! (MTLResourceID :: ENCODING . to_string () , expected) ; } }
};
}
