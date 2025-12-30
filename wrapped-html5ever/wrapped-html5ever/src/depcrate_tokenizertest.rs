// Generated macro for test (module)
macro_rules! Depcrate_tokenizertest {
() => {
// Module: crate::tokenizer
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [allow (non_snake_case)] mod test { use super :: option_push ; use tendril :: { StrTendril , SliceExt } ; # [test] fn push_to_None_gives_singleton () { let mut s : Option < StrTendril > = None ; option_push (& mut s , 'x') ; assert_eq ! (s , Some ("x" . to_tendril ())) ; } # [test] fn push_to_empty_appends () { let mut s : Option < StrTendril > = Some (StrTendril :: new ()) ; option_push (& mut s , 'x') ; assert_eq ! (s , Some ("x" . to_tendril ())) ; } # [test] fn push_to_nonempty_appends () { let mut s : Option < StrTendril > = Some (StrTendril :: from_slice ("y")) ; option_push (& mut s , 'x') ; assert_eq ! (s , Some ("yx" . to_tendril ())) ; } }
};
}
