// Generated macro for Op (struct)
macro_rules! Depcrate_pratt_parserOp {
() => {
// Module: crate::pratt_parser
// Provides: {"Op"}
// Dependencies: {}
# [doc = " An operator that corresponds to a rule."] pub struct Op < R : RuleType > { rule : R , affix : Affix , next : Option < Box < Op < R > > > , }
};
}
