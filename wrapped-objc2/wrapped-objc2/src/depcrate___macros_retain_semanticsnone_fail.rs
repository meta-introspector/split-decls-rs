// Generated macro for none_fail (function)
macro_rules! Depcrate___macros_retain_semanticsnone_fail {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"none_fail"}
// Dependencies: {}
# [cold] # [track_caller] fn none_fail (receiver : Option < & AnyObject > , sel : Sel) -> ! { if let Some (receiver) = receiver { let cls = receiver . class () ; panic ! ("unexpected NULL returned from {}[{cls} {sel}]" , if cls . is_metaclass () { "+" } else { "-" } ,) } else { panic ! ("unexpected NULL {sel}; receiver was NULL") ; } }
};
}
