// Generated macro for def_access_tuple (macro)
macro_rules! Depcrate_strategy_unionsdef_access_tuple {
() => {
// Module: crate::strategy::unions
// Provides: {"def_access_tuple"}
// Dependencies: {}
macro_rules ! def_access_tuple { ($ b : tt $ name : ident , $ ($ n : tt) *) => { macro_rules ! $ name { ([$ b ($ b muta : tt) *] $ b dst : ident = $ b this : expr , $ b ix : expr , $ b body : block) => { match $ b ix { 0 => { let $ b dst = &$ b ($ b muta) * $ b this . options . 0 ; $ b body } , $ ($ n => { if let Some (ref $ b ($ b muta) * $ b dst) = $ b this . options .$ n { $ b body } else { panic ! ("TupleUnion tried to access \
                                        uninitialised slot {}" , $ n) } } ,) * _ => panic ! ("TupleUnion tried to access out-of-range \
                                 slot {}" , $ b ix) , } } } } }
};
}
