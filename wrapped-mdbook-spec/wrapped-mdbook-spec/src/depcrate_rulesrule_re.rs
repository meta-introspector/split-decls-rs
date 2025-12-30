// Generated macro for RULE_RE (static)
macro_rules! Depcrate_rulesRULE_RE {
() => {
// Module: crate::rules
// Provides: {"RULE_RE"}
// Dependencies: {}
# [doc = " The Regex for rules like `r[foo]`."] static RULE_RE : Lazy < Regex > = Lazy :: new (| | Regex :: new (r"(?m)^r\[([^]]+)]$") . unwrap ()) ;
};
}
