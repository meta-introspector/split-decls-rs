// Generated macro for assert_async_wrapio (function)
macro_rules! Depcrate_ioassert_async_wrapio {
() => {
// Module: crate::io
// Provides: {"assert_async_wrapio"}
// Dependencies: {}
fn assert_async_wrapio < F , T > (mut f : F) -> Poll < std :: io :: Result < T > > where F : FnMut () -> std :: io :: Result < T > , { loop { match f () { Err (err) if err . kind () == ErrorKind :: Interrupted => { } res => return Poll :: Ready (res) , } } }
};
}
