// Generated macro for disallow_in_static (function)
macro_rules! Depcrate___macros_classdisallow_in_static {
() => {
// Module: crate::__macros::class
// Provides: {"disallow_in_static"}
// Dependencies: {}
# [doc = " Disallow using this passed in value in const and statics for forwards"] # [doc = " compatibility (this function is not a `const` function)."] # [inline] pub fn disallow_in_static < T > (item : & 'static T) -> & 'static T { item }
};
}
