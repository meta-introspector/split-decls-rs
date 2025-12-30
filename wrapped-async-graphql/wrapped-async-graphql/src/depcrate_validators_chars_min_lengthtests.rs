// Generated macro for tests (module)
macro_rules! Depcrate_validators_chars_min_lengthtests {
() => {
// Module: crate::validators::chars_min_length
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_chars_min_length () { assert ! (chars_min_length (& "你好" . to_string () , 3) . is_err ()) ; assert ! (chars_min_length (& "你好啊" . to_string () , 3) . is_ok ()) ; assert ! (chars_min_length (& "嗨你好啊" . to_string () , 3) . is_ok ()) ; } }
};
}
