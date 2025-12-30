// Generated macro for impl_443 (impl)
macro_rules! Depcrate_msrvsimpl_443 {
() => {
// Module: crate::msrvs
// Provides: {"impl_443"}
// Dependencies: {}
impl MsrvStack { pub fn new (initial : Msrv) -> Self { Self { stack : SmallVec :: from_iter (initial . 0) , } } pub fn current (& self) -> Option < RustcVersion > { self . stack . last () . copied () } pub fn meets (& self , required : RustcVersion) -> bool { self . current () . is_none_or (| msrv | msrv >= required) } pub fn check_attributes (& mut self , sess : & Session , attrs : & [Attribute]) { if let Some (version) = parse_attrs (sess , attrs) { SEEN_MSRV_ATTR . store (true , Ordering :: Relaxed) ; self . stack . push (version) ; } } pub fn check_attributes_post (& mut self , sess : & Session , attrs : & [Attribute]) { if parse_attrs (sess , attrs) . is_some () { self . stack . pop () ; } } }
};
}
