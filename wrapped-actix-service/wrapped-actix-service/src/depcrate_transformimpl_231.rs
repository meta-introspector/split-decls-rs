// Generated macro for impl_231 (impl)
macro_rules! Depcrate_transformimpl_231 {
() => {
// Module: crate::transform
// Provides: {"impl_231"}
// Dependencies: {}
impl < T , S , Req > Transform < S , Req > for Rc < T > where T : Transform < S , Req > , { type Response = T :: Response ; type Error = T :: Error ; type Transform = T :: Transform ; type InitError = T :: InitError ; type Future = T :: Future ; fn new_transform (& self , service : S) -> T :: Future { self . as_ref () . new_transform (service) } }
};
}
