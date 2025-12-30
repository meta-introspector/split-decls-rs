// Generated macro for LocalKey (struct)
macro_rules! Depcrate_threadLocalKey {
() => {
// Module: crate::thread
// Provides: {"LocalKey"}
// Dependencies: {}
# [doc = " Mock implementation of `std::thread::LocalKey`."] pub struct LocalKey < T > { # [doc (hidden)] pub init : fn () -> T , # [doc (hidden)] pub _p : PhantomData < fn (T) > , }
};
}
