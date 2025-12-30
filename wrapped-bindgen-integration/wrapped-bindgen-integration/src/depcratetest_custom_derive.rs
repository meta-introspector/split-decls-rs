// Generated macro for test_custom_derive (function)
macro_rules! Depcratetest_custom_derive {
() => {
// Module: crate
// Provides: {"test_custom_derive"}
// Dependencies: {}
# [test] fn test_custom_derive () { let test1 = unsafe { bindings :: Test :: new (5) } ; let test2 = unsafe { bindings :: Test :: new (6) } ; assert_ne ! (test1 , test2) ; let micron = unsafe { bindings :: MyOrderedEnum :: MICRON } ; let meter = unsafe { bindings :: MyOrderedEnum :: METER } ; let lightyear = unsafe { bindings :: MyOrderedEnum :: LIGHTYEAR } ; assert ! (meter < lightyear) ; assert ! (meter > micron) ; let test1 = unsafe { bindings :: TestDeriveOnAlias (5) } ; let test2 = unsafe { bindings :: TestDeriveOnAlias (6) } ; assert ! (test1 < test2) ; assert ! (! (test1 > test2)) ; }
};
}
