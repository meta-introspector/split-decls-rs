// Generated macro for impl_746 (impl)
macro_rules! Depcrate_frame_utilimpl_746 {
() => {
// Module: crate::frame::util
// Provides: {"impl_746"}
// Dependencies: {}
impl < 'a , 'f : 'a > DebugFlags < 'a , 'f > { pub (super) fn flag_if (& mut self , enabled : bool , name : & str) -> & mut Self { if enabled { self . result = self . result . and_then (| () | { let prefix = if self . started { " | " } else { self . started = true ; ": " } ; write ! (self . fmt , "{}{}" , prefix , name) }) ; } self } pub (super) fn finish (& mut self) -> fmt :: Result { self . result . and_then (| () | write ! (self . fmt , ")")) } }
};
}
