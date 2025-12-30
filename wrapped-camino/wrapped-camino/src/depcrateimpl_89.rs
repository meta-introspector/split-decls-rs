// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl From < & '_ Utf8Path > for Rc < Utf8Path > { fn from (path : & Utf8Path) -> Rc < Utf8Path > { let rc : Rc < Path > = Rc :: from (AsRef :: < Path > :: as_ref (path)) ; let ptr = Rc :: into_raw (rc) as * const Utf8Path ; unsafe { Rc :: from_raw (ptr) } } }
};
}
