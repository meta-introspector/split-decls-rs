// Generated macro for impl_261 (impl)
macro_rules! Depcrate_object_commitimpl_261 {
() => {
// Module: crate::object::commit
// Provides: {"impl_261"}
// Dependencies: {}
# [doc = " Remove Lifetime"] impl Commit < '_ > { # [doc = " Create an owned instance of this object, copying our data in the process."] pub fn detached (& self) -> ObjectDetached { ObjectDetached { id : self . id , kind : gix_object :: Kind :: Commit , data : self . data . clone () , } } # [doc = " Sever the connection to the `Repository` and turn this instance into a standalone object."] pub fn detach (self) -> ObjectDetached { self . into () } # [doc = " Retrieve this instance's encoded data, leaving its own data empty."] # [doc = ""] # [doc = " This method works around the immovability of members of this type."] pub fn take_data (& mut self) -> Vec < u8 > { std :: mem :: take (& mut self . data) } }
};
}
