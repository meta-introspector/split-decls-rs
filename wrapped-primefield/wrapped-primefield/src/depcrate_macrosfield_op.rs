// Generated macro for field_op (macro)
macro_rules! Depcrate_macrosfield_op {
() => {
// Module: crate::macros
// Provides: {"field_op"}
// Dependencies: {}
# [doc = " Emit a `core::ops` trait wrapper for an inherent method which is expected to be provided by a"] # [doc = " backend arithmetic implementation (e.g. `fiat-crypto`)"] # [macro_export] macro_rules ! field_op { ($ fe : path , $ op : tt , $ func : ident , $ inner_func : ident) => { impl :: core :: ops ::$ op for $ fe { type Output = $ fe ; # [inline] fn $ func (self , rhs : $ fe) -> $ fe { <$ fe >::$ inner_func (& self , & rhs) } } impl :: core :: ops ::$ op <&$ fe > for $ fe { type Output = $ fe ; # [inline] fn $ func (self , rhs : &$ fe) -> $ fe { <$ fe >::$ inner_func (& self , rhs) } } impl :: core :: ops ::$ op <&$ fe > for &$ fe { type Output = $ fe ; # [inline] fn $ func (self , rhs : &$ fe) -> $ fe { <$ fe >::$ inner_func (self , rhs) } } } ; }
};
}
