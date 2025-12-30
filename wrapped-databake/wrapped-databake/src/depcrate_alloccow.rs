// Generated macro for cow (function)
macro_rules! Depcrate_alloccow {
() => {
// Module: crate::alloc
// Provides: {"cow"}
// Dependencies: {}
# [test] fn cow () { test_bake ! (alloc :: borrow :: Cow <'static , str >, const , alloc :: borrow :: Cow :: Borrowed ("hi") , alloc) ; assert_eq ! (BakeSize :: borrows_size (& alloc :: borrow :: Cow :: Borrowed ("hi")) , 2) ; assert_eq ! (Bake :: bake (& alloc :: borrow :: Cow ::<'static , str >:: Borrowed ("hi") , & Default :: default () ,) . to_string () , Bake :: bake (& alloc :: borrow :: Cow ::<'static , str >:: Owned ("hi" . to_owned ()) , & Default :: default () ,) . to_string () ,) ; assert_eq ! (BakeSize :: borrows_size (& alloc :: borrow :: Cow ::<'static , str >:: Owned ("hi" . to_owned ())) , 2) ; }
};
}
