// Generated macro for parse_comma_sep (function)
macro_rules! Depcrate_attrparse_comma_sep {
() => {
// Module: crate::attr
// Provides: {"parse_comma_sep"}
// Dependencies: {}
fn parse_comma_sep < S > (iter : TtIter < '_ , S >) -> Vec < Symbol > { iter . filter_map (| tt | match tt { TtElement :: Leaf (tt :: Leaf :: Literal (tt :: Literal { kind : tt :: LitKind :: Str , symbol , .. })) => Some (symbol . clone ()) , _ => None , }) . collect () }
};
}
