// Generated macro for impl_368 (impl)
macro_rules! Depcrate_objectimpl_368 {
() => {
// Module: crate::object
// Provides: {"impl_368"}
// Dependencies: {}
impl Object < '_ > { # [doc = " Create an owned instance of this object, copying our data in the process."] pub fn detached (& self) -> ObjectDetached { ObjectDetached { id : self . id , kind : self . kind , data : self . data . clone () , } } # [doc = " Sever the connection to the `Repository` and turn this instance into a standalone object."] pub fn detach (self) -> ObjectDetached { self . into () } }
};
}
