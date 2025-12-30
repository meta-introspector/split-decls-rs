// Generated macro for build_foo_err (function)
macro_rules! Depcratebuild_foo_err {
() => {
// Module: crate
// Provides: {"build_foo_err"}
// Dependencies: {}
pub fn build_foo_err () -> Option < FooError > { let foo = FooBuilder :: default () . build () ; foo . err () }
};
}
