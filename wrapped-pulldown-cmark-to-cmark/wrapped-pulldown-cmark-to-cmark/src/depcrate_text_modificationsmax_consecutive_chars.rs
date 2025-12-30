// Generated macro for max_consecutive_chars (module)
macro_rules! Depcrate_text_modificationsmax_consecutive_chars {
() => {
// Module: crate::text_modifications
// Provides: {"max_consecutive_chars"}
// Dependencies: {}
# [cfg (test)] mod max_consecutive_chars { use super :: max_consecutive_chars ; # [test] fn happens_in_the_entire_string () { assert_eq ! (max_consecutive_chars ("``a```b``" , '`') , 3 , "the highest seen consecutive segment of backticks counts") ; assert_eq ! (max_consecutive_chars ("```a``b`" , '`') , 3 , "it can't be downgraded later") ; } }
};
}
