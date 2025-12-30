// Generated macro for tests (module)
macro_rules! Depcrate_validators_chars_max_lengthtests {
() => {
// Module: crate::validators::chars_max_length
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_chars_max_length () { assert ! (chars_max_length (& "你好" . to_string () , 3) . is_ok ()) ; assert ! (chars_max_length (& "你好啊" . to_string () , 3) . is_ok ()) ; assert ! (chars_max_length (& "嗨你好啊" . to_string () , 3) . is_err ()) ; } }
};
}
