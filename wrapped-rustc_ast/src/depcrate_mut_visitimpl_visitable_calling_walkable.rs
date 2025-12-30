// Generated macro for impl_visitable_calling_walkable (macro)
macro_rules! Depcrate_mut_visitimpl_visitable_calling_walkable {
() => {
// Module: crate::mut_visit
// Provides: {"impl_visitable_calling_walkable"}
// Dependencies: {}
macro_rules ! impl_visitable_calling_walkable { (< mut > $ (fn $ method : ident ($ ty : ty $ (, $ extra_name : ident : $ extra_ty : ty) ?) ;) *) => { $ (fn $ method (& mut self , node : & mut $ ty $ (, $ extra_name :$ extra_ty) ?) { impl_visitable ! (|& mut self : $ ty , visitor : & mut V , extra : ($ ($ extra_ty) ?) | { let ($ ($ extra_name) ?) = extra ; visitor .$ method (self $ (, $ extra_name) ?) ; }) ; walk_walkable ! (self , node , mut) }) * } }
};
}
