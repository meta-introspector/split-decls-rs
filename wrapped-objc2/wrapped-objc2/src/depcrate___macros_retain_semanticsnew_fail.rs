// Generated macro for new_fail (function)
macro_rules! Depcrate___macros_retain_semanticsnew_fail {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"new_fail"}
// Dependencies: {}
# [cold] # [track_caller] fn new_fail (receiver : Option < & AnyObject > , sel : Sel) -> ! { if let Some (receiver) = receiver { let cls = receiver . class () ; if cls . is_metaclass () { if sel == sel ! (new) { panic ! ("failed creating new instance of {cls}") } else { panic ! ("failed creating new instance using +[{cls} {sel}]") } } else { panic ! ("unexpected NULL returned from -[{cls} {sel}]") } } else { panic ! ("unexpected NULL {sel}; receiver was NULL") ; } }
};
}
