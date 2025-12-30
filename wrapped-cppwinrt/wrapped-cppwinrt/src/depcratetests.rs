// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: * ; # [test] # [should_panic (expected = "'-invalid' is not supported")] fn invalid_arg () { cppwinrt (["-invalid"]) ; } # [test] fn unexpected_version () { let ok = cppwinrt (["-help"]) ; assert ! (ok . contains ("2.0.250303.1") , "unexpected version") ; } }
};
}
