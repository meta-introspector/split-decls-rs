// Generated macro for find_continuable (function)
macro_rules! Depcrate_inferfind_continuable {
() => {
// Module: crate::infer
// Provides: {"find_continuable"}
// Dependencies: {}
fn find_continuable < 'a , 'db > (ctxs : & 'a mut [BreakableContext < 'db >] , label : Option < LabelId > ,) -> Option < & 'a mut BreakableContext < 'db > > { match label { Some (_) => find_breakable (ctxs , label) . filter (| it | matches ! (it . kind , BreakableKind :: Loop)) , None => find_breakable (ctxs , label) , } }
};
}
