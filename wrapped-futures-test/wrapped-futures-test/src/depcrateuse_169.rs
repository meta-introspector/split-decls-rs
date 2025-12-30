// Generated macro for use_169 (pub_use)
macro_rules! Depcrateuse_169 {
() => {
// Module: crate
// Provides: {"use_169"}
// Dependencies: {}
# [doc = " Enables an `async` test function. The generated future will be run to completion with"] # [doc = " [`futures_executor::block_on`]."] # [doc = ""] # [doc = " ```"] # [doc = " #[futures_test::test]"] # [doc = " async fn my_test() {"] # [doc = "     let fut = async { true };"] # [doc = "     assert!(fut.await);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This is equivalent to the following code:"] # [doc = ""] # [doc = " ```"] # [doc = " #[test]"] # [doc = " fn my_test() {"] # [doc = "     futures::executor::block_on(async move {"] # [doc = "         let fut = async { true };"] # [doc = "         assert!(fut.await);"] # [doc = "     })"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "std")] pub use futures_macro :: test_internal as test ;
};
}
