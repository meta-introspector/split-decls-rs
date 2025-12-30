// Generated macro for test (module)
macro_rules! Depcrate_blocktest {
() => {
// Module: crate::block
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use std :: convert :: TryInto ; use super :: * ; fn parse (s : & str) -> Result < BlockContents , syn :: Error > { (& LitStr :: new (s , Span :: call_site ())) . try_into () } # [test] # [should_panic (expected = r#"cannot parse"#)] fn block_invalid_token_trees () { parse ("let x = 2; { x+1") . unwrap () ; } # [test] fn block_delimited_token_tree () { let expr = parse ("let x = 2; { x+1 }") . unwrap () ; assert_eq ! (quote ! (# expr) . to_string () , quote ! ({ let x = 2 ; { x + 1 } }) . to_string ()) ; } # [test] fn block_single_token_tree () { let expr = parse ("42") . unwrap () ; assert_eq ! (quote ! (# expr) . to_string () , quote ! ({ 42 }) . to_string ()) ; } }
};
}
