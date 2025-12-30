// Generated macro for tests (module)
macro_rules! Depcrate_typestests {
() => {
// Module: crate::types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: LazyPyImport ; # [test] fn test_basic () { pyo3 :: Python :: initialize () ; let v = LazyPyImport :: new ("foo" , & ["bar"]) ; pyo3 :: Python :: attach (| py | { assert ! (v . get (py) . is_err ()) ; }) ; } }
};
}
