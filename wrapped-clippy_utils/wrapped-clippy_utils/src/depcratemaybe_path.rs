// Generated macro for maybe_path (macro)
macro_rules! Depcratemaybe_path {
() => {
// Module: crate
// Provides: {"maybe_path"}
// Dependencies: {}
macro_rules ! maybe_path { ($ ty : ident , $ kind : ident) => { impl <'hir > MaybePath <'hir > for hir ::$ ty <'hir > { fn hir_id (& self) -> HirId { self . hir_id } fn qpath_opt (& self) -> Option <& QPath <'hir >> { match & self . kind { hir ::$ kind :: Path (qpath) => Some (qpath) , _ => None , } } } } ; }
};
}
