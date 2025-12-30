// Generated macro for test_macro (function)
macro_rules! Depcratetest_macro {
() => {
// Module: crate
// Provides: {"test_macro"}
// Dependencies: {}
# [test] fn test_macro () { use core :: cell :: RefCell ; let i = RefCell :: new (0) ; let k = RefCell :: new (0) ; { defer ! (* i . borrow_mut () += 1) ; defer ! (* k . borrow_mut () += 1) ; assert_eq ! (* i . borrow () , 0) ; assert_eq ! (* k . borrow () , 0) ; } assert_eq ! (* i . borrow () , 1) ; assert_eq ! (* k . borrow () , 1) ; }
};
}
