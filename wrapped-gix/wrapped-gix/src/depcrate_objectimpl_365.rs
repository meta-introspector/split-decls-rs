// Generated macro for impl_365 (impl)
macro_rules! Depcrate_objectimpl_365 {
() => {
// Module: crate::object
// Provides: {"impl_365"}
// Dependencies: {}
impl ObjectDetached { # [doc = " Infuse this owned object with `repo` access."] pub fn attach (self , repo : & crate :: Repository) -> Object < '_ > { Object { id : self . id , kind : self . kind , data : self . data , repo , } } }
};
}
