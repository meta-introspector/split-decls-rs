// Generated macro for impl_617 (impl)
macro_rules! Depcrate_typesimpl_617 {
() => {
// Module: crate::types
// Provides: {"impl_617"}
// Dependencies: {}
impl LazyPyImport { pub const fn new (module : & 'static str , names : & 'static [& 'static str]) -> LazyPyImport { LazyPyImport { module , names , value : pyo3 :: sync :: PyOnceLock :: new () , } } pub fn get < 'p > (& 'p self , py : pyo3 :: Python < 'p >) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let p = self . value . get_or_try_init (py , | | { let mut obj = py . import (self . module) ? . into_any () ; for name in self . names { obj = obj . getattr (* name) ? ; } Ok :: < _ , pyo3 :: PyErr > (obj . unbind ()) }) ? ; Ok (p . clone_ref (py) . into_bound (py)) } }
};
}
