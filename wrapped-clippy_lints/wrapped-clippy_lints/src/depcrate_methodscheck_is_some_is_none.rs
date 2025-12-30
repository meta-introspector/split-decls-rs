// Generated macro for check_is_some_is_none (function)
macro_rules! Depcrate_methodscheck_is_some_is_none {
() => {
// Module: crate::methods
// Provides: {"check_is_some_is_none"}
// Dependencies: {}
fn check_is_some_is_none (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , call_span : Span , is_some : bool) { match method_call (recv) { Some ((name @ (sym :: find | sym :: position | sym :: rposition) , f_recv , [arg] , span , _)) => { search_is_some :: check (cx , expr , name , is_some , f_recv , arg , recv , span) ; } , Some ((sym :: get , f_recv , [arg] , _ , _)) => { unnecessary_get_then_check :: check (cx , call_span , recv , f_recv , arg , is_some) ; } , Some ((sym :: first , f_recv , [] , _ , _)) => { unnecessary_first_then_check :: check (cx , call_span , recv , f_recv , is_some) ; } , _ => { } , } }
};
}
