// Generated macro for CLOSURE (static)
macro_rules! Depcrate_scan_exprCLOSURE {
() => {
// Module: crate::scan_expr
// Provides: {"CLOSURE"}
// Dependencies: {}
static CLOSURE : [(Input , Action) ; 6] = [(Keyword ("async") , SetState (& CLOSURE)) , (Keyword ("move") , SetState (& CLOSURE)) , (Punct (",") , SetState (& CLOSURE)) , (Punct (">") , SetState (& CLOSURE)) , (Punct ("|") , SetState (& CLOSURE_ARGS)) , (ConsumeLifetime , SetState (& CLOSURE)) ,] ;
};
}
