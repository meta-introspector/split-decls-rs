// Generated macro for contains_cfg_arm (function)
macro_rules! Depcrate_matchescontains_cfg_arm {
() => {
// Module: crate::matches
// Provides: {"contains_cfg_arm"}
// Dependencies: {}
# [doc = " Checks if there are any arms with a `#[cfg(..)]` attribute."] fn contains_cfg_arm (cx : & LateContext < '_ > , e : & Expr < '_ > , scrutinee : & Expr < '_ > , arms : & [Arm < '_ >]) -> bool { let Some (scrutinee_span) = walk_span_to_context (scrutinee . span , SyntaxContext :: root ()) else { return true ; } ; let start = scrutinee_span . hi () ; let mut arm_spans = arms . iter () . map (| arm | { let data = arm . span . data () ; (data . ctxt == SyntaxContext :: root ()) . then_some ((data . lo , data . hi)) }) ; let end = e . span . hi () ; let found = arm_spans . try_fold (start , | start , range | { let Some ((end , next_start)) = range else { return Err (()) ; } ; let span = SpanData { lo : start , hi : end , ctxt : SyntaxContext :: root () , parent : None , } . span () ; (! span_contains_cfg (cx , span)) . then_some (next_start) . ok_or (()) }) ; match found { Ok (start) => { let span = SpanData { lo : start , hi : end , ctxt : SyntaxContext :: root () , parent : None , } . span () ; span_contains_cfg (cx , span) } , Err (()) => true , } }
};
}
