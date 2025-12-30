// Generated macro for tests (module)
macro_rules! Depcrate_formattests {
() => {
// Module: crate::format
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn eq_str_display_1 () { assert ! (eq_str_display ("hello" , "hello")) ; assert ! (eq_str_display ("42" , & 42)) ; assert ! (eq_str_display (r#"\x00\t\r\n\xff\\"# , & b"\x00\t\r\n\xff\\" . escape_ascii ())) ; assert ! (! eq_str_display ("hello" , "world")) ; assert ! (! eq_str_display ("hello world" , "hello")) ; assert ! (! eq_str_display ("hello" , "hello world")) ; assert ! (! eq_str_display ("42" , & 4)) ; assert ! (! eq_str_display ("4" , & 42)) ; } }
};
}
