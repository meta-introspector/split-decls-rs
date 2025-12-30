// Generated macro for loop_init_suggestion (function)
macro_rules! Depcrate_rc_clone_in_vec_initloop_init_suggestion {
() => {
// Module: crate::rc_clone_in_vec_init
// Provides: {"loop_init_suggestion"}
// Dependencies: {}
fn loop_init_suggestion (elem : & str , len : & str , indent : & str) -> String { format ! (r"{{
{indent}    let mut v = Vec::with_capacity({len});
{indent}    (0..{len}).for_each(|_| v.push({elem}));
{indent}    v
{indent}}}") }
};
}
