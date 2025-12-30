// Generated macro for macro_15 (macro)
macro_rules! Depcrate_macosmacro_15 {
() => {
// Module: crate::macos
// Provides: {"macro_15"}
// Dependencies: {}
lazy_static ! { # [doc = " A lock protecting dyld FFI calls."] # [doc = ""] # [doc = " MacOS does not provide an atomic way to iterate shared libraries, so"] # [doc = " *you* must take this lock whenever dynamically adding or removing shared"] # [doc = " libraries to ensure that there are no races with iterating shared"] # [doc = " libraries."] pub static ref DYLD_LOCK : Mutex < () > = Mutex :: new (()) ; }
};
}
