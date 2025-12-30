// Generated macro for prevent_inline_duplicate (function)
macro_rules! Depcrate_runtimeprevent_inline_duplicate {
() => {
// Module: crate::runtime
// Provides: {"prevent_inline_duplicate"}
// Dependencies: {}
fn prevent_inline_duplicate (function_name : & str , assertion_file : & str , assertion_line : u32) { let key = format ! ("{function_name}|{assertion_file}|{assertion_line}") ; let mut set = INLINE_DUPLICATES . lock () . unwrap () ; if set . contains (& key) { drop (set) ; panic ! ("Insta does not allow inline snapshot assertions in loops. \
            Wrap your assertions in allow_duplicates! to change this.") ; } set . insert (key) ; }
};
}
