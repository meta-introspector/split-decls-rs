// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () { use core :: cell :: RefCell ; let i = RefCell :: new (0) ; { let _d = defer (| | * i . borrow_mut () += 1) ; assert_eq ! (* i . borrow () , 0) ; } assert_eq ! (* i . borrow () , 1) ; }
};
}
