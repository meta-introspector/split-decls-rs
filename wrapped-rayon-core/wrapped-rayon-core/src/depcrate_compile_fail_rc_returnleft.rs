// Generated macro for left (module)
macro_rules! Depcrate_compile_fail_rc_returnleft {
() => {
// Module: crate::compile_fail::rc_return
// Provides: {"left"}
// Dependencies: {}
# [doc = " ```compile_fail,E0277\n\nuse std::rc::Rc;\n\nrayon_core::join(|| Rc::new(22), || ()); //~ ERROR\n\n``` "] mod left { }
};
}
