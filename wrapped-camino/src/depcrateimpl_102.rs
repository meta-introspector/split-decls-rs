// Generated macro for impl_102 (impl)
macro_rules! Depcrateimpl_102 {
() => {
// Module: crate
// Provides: {"impl_102"}
// Dependencies: {}
impl From < Utf8PathBuf > for Rc < Utf8Path > { fn from (path : Utf8PathBuf) -> Rc < Utf8Path > { let rc : Rc < Path > = Rc :: from (path . 0) ; let ptr = Rc :: into_raw (rc) as * const Utf8Path ; unsafe { Rc :: from_raw (ptr) } } }
};
}
