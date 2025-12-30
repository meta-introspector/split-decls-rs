// Generated macro for impl_88 (impl)
macro_rules! Depcrate_hello_worldimpl_88 {
() => {
// Module: crate::hello_world
// Provides: {"impl_88"}
// Dependencies: {}
impl DataPayload < HelloWorldV1 > { # [doc = " Make a [`DataPayload`]`<`[`HelloWorldV1`]`>` from a static string slice."] pub fn from_static_str (s : & 'static str) -> DataPayload < HelloWorldV1 > { DataPayload :: from_owned (HelloWorld { message : Cow :: Borrowed (s) , }) } }
};
}
