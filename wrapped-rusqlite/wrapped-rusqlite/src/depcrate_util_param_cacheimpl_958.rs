// Generated macro for impl_958 (impl)
macro_rules! Depcrate_util_param_cacheimpl_958 {
() => {
// Module: crate::util::param_cache
// Provides: {"impl_958"}
// Dependencies: {}
impl ParamIndexCache { pub fn get_or_insert_with < F > (& self , s : & str , func : F) -> Option < usize > where F : FnOnce (& std :: ffi :: CStr) -> Option < usize > , { let mut cache = self . 0 . borrow_mut () ; if let Some (v) = cache . get (s) { return Some (* v) ; } let name = SmallCString :: new (s) . ok () ? ; let val = func (& name) ? ; cache . insert (name , val) ; Some (val) } }
};
}
