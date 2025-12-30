// Generated macro for impl_253 (impl)
macro_rules! Depcrate_use_trackingimpl_253 {
() => {
// Module: crate::use_tracking
// Provides: {"impl_253"}
// Dependencies: {}
impl UseMarkable for syn :: Type { fn mark_uses (& self , ut : & mut UseTracker) { use syn :: visit ; visit :: visit_type (& mut PathVisitor (ut) , self) ; struct PathVisitor < 'ut > (& 'ut mut UseTracker) ; impl < 'ut , 'ast > visit :: Visit < 'ast > for PathVisitor < 'ut > { fn visit_macro (& mut self , _ : & syn :: Macro) { } fn visit_type_path (& mut self , tpath : & syn :: TypePath) { if matches_prj_tyvar (self . 0 , tpath) { self . 0 . use_type (adjust_simple_prj (tpath) . into ()) ; return ; } visit :: visit_type_path (self , tpath) ; } fn visit_path (& mut self , path : & syn :: Path) { if util :: is_phantom_data (path) { return ; } if let Some (ident) = util :: extract_simple_path (path) { self . 0 . use_tyvar (ident) ; } visit :: visit_path (self , path) ; } } } }
};
}
