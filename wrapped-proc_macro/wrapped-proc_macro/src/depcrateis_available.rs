// Generated macro for is_available (function)
macro_rules! Depcrateis_available {
() => {
// Module: crate
// Provides: {"is_available"}
// Dependencies: {}
# [doc = " Determines whether proc_macro has been made accessible to the currently"] # [doc = " running program."] # [doc = ""] # [doc = " The proc_macro crate is only intended for use inside the implementation of"] # [doc = " procedural macros. All the functions in this crate panic if invoked from"] # [doc = " outside of a procedural macro, such as from a build script or unit test or"] # [doc = " ordinary Rust binary."] # [doc = ""] # [doc = " With consideration for Rust libraries that are designed to support both"] # [doc = " macro and non-macro use cases, `proc_macro::is_available()` provides a"] # [doc = " non-panicking way to detect whether the infrastructure required to use the"] # [doc = " API of proc_macro is presently available. Returns true if invoked from"] # [doc = " inside of a procedural macro, false if invoked from any other binary."] # [stable (feature = "proc_macro_is_available" , since = "1.57.0")] pub fn is_available () -> bool { bridge :: client :: is_available () }
};
}
