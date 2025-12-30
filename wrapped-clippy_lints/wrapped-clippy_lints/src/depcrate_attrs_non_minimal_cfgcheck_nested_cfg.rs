// Generated macro for check_nested_cfg (function)
macro_rules! Depcrate_attrs_non_minimal_cfgcheck_nested_cfg {
() => {
// Module: crate::attrs::non_minimal_cfg
// Provides: {"check_nested_cfg"}
// Dependencies: {}
fn check_nested_cfg (cx : & EarlyContext < '_ > , items : & [MetaItemInner]) { for item in items { if let MetaItemInner :: MetaItem (meta) = item { if ! meta . has_name (sym :: any) && ! meta . has_name (sym :: all) { continue ; } if let MetaItemKind :: List (list) = & meta . kind { check_nested_cfg (cx , list) ; if list . len () == 1 { span_lint_and_then (cx , NON_MINIMAL_CFG , meta . span , "unneeded sub `cfg` when there is only one condition" , | diag | { if let Some (snippet) = list [0] . span () . get_source_text (cx) { diag . span_suggestion (meta . span , "try" , snippet . to_owned () , Applicability :: MaybeIncorrect ,) ; } } ,) ; } else if list . is_empty () && meta . has_name (sym :: all) { span_lint_and_then (cx , NON_MINIMAL_CFG , meta . span , "unneeded sub `cfg` when there is no condition" , | _ | { } ,) ; } } } } }
};
}
