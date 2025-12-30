// Generated macro for CALL_STACK_CHILDREN_THRESHOLD (const)
macro_rules! Depcrate_parser_stateCALL_STACK_CHILDREN_THRESHOLD {
() => {
// Module: crate::parser_state
// Provides: {"CALL_STACK_CHILDREN_THRESHOLD"}
// Dependencies: {}
# [doc = " Max rule children number for which we'll extend calls stacks."] # [doc = ""] # [doc = " In case rule we're working with has too many children rules that failed in parsing,"] # [doc = " we don't want to store long stacks for all of them. If rule has more than this number"] # [doc = " of failed children, they all will be collapsed in a parent rule."] const CALL_STACK_CHILDREN_THRESHOLD : usize = 4 ;
};
}
