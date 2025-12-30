// Generated macro for impl_391 (impl)
macro_rules! Depcrate_prop_nameimpl_391 {
() => {
// Module: crate::prop_name
// Provides: {"impl_391"}
// Dependencies: {}
impl std :: ops :: Deref for PropertyName { type Target = PropName ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { & * (ptr :: from_ref :: < CStr > (self . 0 . as_c_str ()) as * const PropName) } } }
};
}
