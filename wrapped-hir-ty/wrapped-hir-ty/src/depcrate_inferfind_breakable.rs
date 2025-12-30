// Generated macro for find_breakable (function)
macro_rules! Depcrate_inferfind_breakable {
() => {
// Module: crate::infer
// Provides: {"find_breakable"}
// Dependencies: {}
fn find_breakable < 'a , 'db > (ctxs : & 'a mut [BreakableContext < 'db >] , label : Option < LabelId > ,) -> Option < & 'a mut BreakableContext < 'db > > { let mut ctxs = ctxs . iter_mut () . rev () . take_while (| it | matches ! (it . kind , BreakableKind :: Block | BreakableKind :: Loop)) ; match label { Some (_) => ctxs . find (| ctx | ctx . label == label) , None => ctxs . find (| ctx | matches ! (ctx . kind , BreakableKind :: Loop)) , } }
};
}
