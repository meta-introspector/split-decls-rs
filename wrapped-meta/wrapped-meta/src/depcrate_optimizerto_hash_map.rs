// Generated macro for to_hash_map (macro)
macro_rules! Depcrate_optimizerto_hash_map {
() => {
// Module: crate::optimizer
// Provides: {"to_hash_map"}
// Dependencies: {}
macro_rules ! to_hash_map { ($ func_name : ident , $ rule : ty , $ expr : ty) => { fn $ func_name (rules : & [$ rule]) -> HashMap < String , $ expr > { rules . iter () . map (| r | (r . name . clone () , r . expr . clone ())) . collect () } } ; }
};
}
