// Generated macro for impl_288 (impl)
macro_rules! Depcrate_object_tagimpl_288 {
() => {
// Module: crate::object::tag
// Provides: {"impl_288"}
// Dependencies: {}
# [doc = " Remove Lifetime"] impl Tag < '_ > { # [doc = " Create an owned instance of this object, copying our data in the process."] pub fn detached (& self) -> ObjectDetached { ObjectDetached { id : self . id , kind : gix_object :: Kind :: Tag , data : self . data . clone () , } } # [doc = " Sever the connection to the `Repository` and turn this instance into a standalone object."] pub fn detach (self) -> ObjectDetached { self . into () } # [doc = " Retrieve this instance's encoded data, leaving its own data empty."] # [doc = ""] # [doc = " This method works around the immovability of members of this type."] pub fn take_data (& mut self) -> Vec < u8 > { std :: mem :: take (& mut self . data) } }
};
}
