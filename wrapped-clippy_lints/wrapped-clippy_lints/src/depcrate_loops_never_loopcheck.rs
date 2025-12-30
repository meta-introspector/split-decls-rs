// Generated macro for check (function)
macro_rules! Depcrate_loops_never_loopcheck {
() => {
// Module: crate::loops::never_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , block : & Block < 'tcx > , loop_id : HirId , span : Span , for_loop : Option < & ForLoop < '_ > > ,) { match never_loop_block (cx , block , & mut Vec :: new () , loop_id) { NeverLoopResult :: Diverging { ref break_spans , ref never_spans , } => { span_lint_and_then (cx , NEVER_LOOP , span , "this loop never actually loops" , | diag | { if let Some (ForLoop { arg : iterator , pat , span : for_span , label , .. }) = for_loop { let mut app = if ! contains_any_break_or_continue (block) && label . is_none () { Applicability :: MachineApplicable } else { Applicability :: Unspecified } ; if ! never_spans . is_empty () { app = Applicability :: HasPlaceholders ; } let mut suggestions = vec ! [(for_span . with_hi (iterator . span . hi ()) , for_to_if_let_sugg (cx , iterator , pat) ,)] ; suggestions . extend (break_spans . iter () . map (| span | (* span , String :: new ()))) ; diag . multipart_suggestion_verbose ("if you need the first element of the iterator, try writing" , suggestions , app ,) ; for span in never_spans { diag . span_help (* span , "this code is unreachable. Consider moving the reachable parts out" ,) ; } } }) ; } , NeverLoopResult :: MayContinueMainLoop | NeverLoopResult :: Normal => () , } }
};
}
