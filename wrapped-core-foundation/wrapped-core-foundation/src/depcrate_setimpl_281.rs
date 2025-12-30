// Generated macro for impl_281 (impl)
macro_rules! Depcrate_setimpl_281 {
() => {
// Module: crate::set
// Provides: {"impl_281"}
// Dependencies: {}
impl CFSet { # [doc = " Creates a new set from a list of `CFType` instances."] pub fn from_slice < T > (elems : & [T]) -> CFSet < T > where T : TCFType , { unsafe { let elems : Vec < CFTypeRef > = elems . iter () . map (| elem | elem . as_CFTypeRef ()) . collect () ; let set_ref = CFSetCreate (kCFAllocatorDefault , elems . as_ptr () , elems . len () . to_CFIndex () , & kCFTypeSetCallBacks ,) ; TCFType :: wrap_under_create_rule (set_ref) } } }
};
}
