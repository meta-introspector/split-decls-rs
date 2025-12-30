// Generated macro for LazyPyImport (struct)
macro_rules! Depcrate_typesLazyPyImport {
() => {
// Module: crate::types
// Provides: {"LazyPyImport"}
// Dependencies: {}
pub struct LazyPyImport { module : & 'static str , names : & 'static [& 'static str] , value : pyo3 :: sync :: PyOnceLock < pyo3 :: Py < pyo3 :: PyAny > > , }
};
}
