// Generated macro for impl_417 (impl)
macro_rules! Depcrate_consteval_tests_traitsimpl_417 {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"impl_417"}
// Dependencies: {}
impl fmt :: Display for FnTrait { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { FnTrait :: FnOnce => write ! (f , "FnOnce") , FnTrait :: FnMut => write ! (f , "FnMut") , FnTrait :: Fn => write ! (f , "Fn") , FnTrait :: AsyncFnOnce => write ! (f , "AsyncFnOnce") , FnTrait :: AsyncFnMut => write ! (f , "AsyncFnMut") , FnTrait :: AsyncFn => write ! (f , "AsyncFn") , } } }
};
}
