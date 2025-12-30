// Generated macro for common_method_name (function)
macro_rules! Depcrate_com_objectcommon_method_name {
() => {
// Module: crate::com_object
// Provides: {"common_method_name"}
// Dependencies: {}
# [test] fn common_method_name () { let app = MyApp :: new (42) ; let ifoo : IFoo = app . to_interface () ; assert_eq ! (unsafe { ifoo . common () } , 100) ; let ibar : IBar = app . to_interface () ; assert_eq ! (unsafe { ibar . common () } , 1_000_000_000_000) ; let ibar2 : IBar2 = app . to_interface () ; assert_eq ! (unsafe { ibar2 . common () } , std :: f32 :: consts :: PI) ; }
};
}
