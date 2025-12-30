// Generated macro for tests (module)
macro_rules! Depcrate_grammartests {
() => {
// Module: crate::grammar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use super :: * ; # [doc = " Empty label is OK."] # [test] fn split_label_empty () { let (label , body) = split_label (b"-----\nBODY") . unwrap () ; assert_eq ! (label , "") ; assert_eq ! (body , b"BODY") ; } # [doc = " Label containing text."] # [test] fn split_label_with_text () { let (label , body) = split_label (b"PRIVATE KEY-----\nBODY") . unwrap () ; assert_eq ! (label , "PRIVATE KEY") ; assert_eq ! (body , b"BODY") ; } # [doc = " Reject labels containing repeated spaces"] # [test] fn split_label_with_repeat_wsp_is_err () { assert ! (split_label (b"PRIVATE  KEY-----\nBODY") . is_none ()) ; } # [doc = " Basic validation of a label"] # [test] fn validate_private_key_label () { assert_eq ! (validate_label (b"PRIVATE KEY") , Ok (())) ; } # [doc = " Reject labels with double spaces"] # [test] fn validate_private_key_label_reject_double_space () { assert_eq ! (validate_label (b"PRIVATE  KEY") , Err (Error :: Label)) ; } }
};
}
