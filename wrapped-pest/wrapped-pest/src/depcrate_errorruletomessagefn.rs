// Generated macro for RuleToMessageFn (type)
macro_rules! Depcrate_errorRuleToMessageFn {
() => {
// Module: crate::error
// Provides: {"RuleToMessageFn"}
// Dependencies: {}
# [doc = " Function mapping rule to its helper message defined by user."] pub type RuleToMessageFn < R > = Box < dyn Fn (& R) -> Option < String > > ;
};
}
