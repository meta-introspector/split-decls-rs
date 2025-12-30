// Generated macro for any_debug (function)
macro_rules! Depcrate_core_builder_testsany_debug {
() => {
// Module: crate::core::builder::tests
// Provides: {"any_debug"}
// Dependencies: {}
# [doc = " The `AnyDebug` trait should delegate to the underlying type's `Debug`, and"] # [doc = " should also allow downcasting as expected."] # [test] fn any_debug () { # [derive (Debug , PartialEq , Eq)] struct MyStruct { x : u32 , } let x : & dyn AnyDebug = & MyStruct { x : 7 } ; assert_eq ! (format ! ("{x:?}") , format ! ("{:?}" , MyStruct { x : 7 })) ; assert_eq ! (x . downcast_ref ::< MyStruct > () , Some (& MyStruct { x : 7 })) ; }
};
}
