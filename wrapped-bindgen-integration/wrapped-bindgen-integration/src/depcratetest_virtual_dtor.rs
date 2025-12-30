// Generated macro for test_virtual_dtor (function)
macro_rules! Depcratetest_virtual_dtor {
() => {
// Module: crate
// Provides: {"test_virtual_dtor"}
// Dependencies: {}
# [test] fn test_virtual_dtor () { unsafe { { let b = bindings :: InheritsFromVirtualDestructor :: new () ; } assert_eq ! (bindings :: InheritsFromVirtualDestructor_sDestructorCount , 1) ; assert_eq ! (bindings :: VirtualDestructor_sDestructorCount , 1) ; } }
};
}
