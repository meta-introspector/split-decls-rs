// Generated macro for product_type (macro)
macro_rules! Depcrate_product_tupleproduct_type {
() => {
// Module: crate::product_tuple
// Provides: {"product_type"}
// Dependencies: {}
macro_rules ! product_type { ($ factor : ty) => { ($ factor ,) } ; ($ ($ factor : ty) ,*) => { ($ ($ factor ,) *) } ; ($ ($ factor : ty) ,*,) => { ($ ($ factor ,) *) } ; }
};
}
