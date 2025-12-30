// Generated macro for define_id_collections (macro)
macro_rules! Depcrate_fxdefine_id_collections {
() => {
// Module: crate::fx
// Provides: {"define_id_collections"}
// Dependencies: {}
# [macro_export] macro_rules ! define_id_collections { ($ map_name : ident , $ set_name : ident , $ entry_name : ident , $ key : ty) => { pub type $ map_name < T > = $ crate :: unord :: UnordMap <$ key , T >; pub type $ set_name = $ crate :: unord :: UnordSet <$ key >; pub type $ entry_name <'a , T > = $ crate :: fx :: StdEntry <'a , $ key , T >; } ; }
};
}
