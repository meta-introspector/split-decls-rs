// Generated macro for other_5 (other)
macro_rules! Depcrateother_5 {
() => {
// Module: crate
// Provides: {"other_5"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Call the given function inside an Objective-C `@try/@catch` block."] # [doc = ""] # [doc = " Defined in `src/try_catch.m` and compiled in `build.rs`."] # [doc = ""] # [doc = " Alternatively, we could manually write assembly for this function like"] # [doc = " [`objrs` does][manual-asm] does, that would cut down on a build stage"] # [doc = " (and would probably give us a bit better performance), but it gets"] # [doc = " unwieldy _very_ quickly, so I chose the much more stable option."] # [doc = ""] # [doc = " Another thing to remember: While Rust's and Objective-C's unwinding"] # [doc = " mechanisms are similar now, Rust's is explicitly unspecified, and they"] # [doc = " may diverge significantly in the future; so handling this in pure Rust"] # [doc = " (using mechanisms like core::intrinsics::r#try) is not an option!"] # [doc = ""] # [doc = " [manual-asm]: https://gitlab.com/objrs/objrs/-/blob/b4f6598696b3fa622e6fddce7aff281770b0a8c2/src/exception.rs"] # [doc = ""] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics / continues unwinding if the unwind is not triggered by an"] # [doc = " Objective-C exception (i.e. it was triggered by Rust/C++/...)."] # [link_name = "objc2_exception_helper_0_1_try_catch"] pub fn try_catch (f : TryCatchClosure , context : * mut c_void , error : * mut * mut c_void) -> u8 ; }
};
}
