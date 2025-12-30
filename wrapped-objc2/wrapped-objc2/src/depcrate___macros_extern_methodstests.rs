// Generated macro for tests (module)
macro_rules! Depcrate___macros_extern_methodstests {
() => {
// Module: crate::__macros::extern_methods
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: runtime :: { NSObject , NSObjectProtocol } ; # [test] fn outside_impl_using_this () { extern_methods ! (# [unsafe (method (hash))] fn obj_hash (this : & NSObject) -> usize ;) ; let obj = NSObject :: new () ; assert_eq ! (obj_hash (& obj) , obj . hash ()) } }
};
}
