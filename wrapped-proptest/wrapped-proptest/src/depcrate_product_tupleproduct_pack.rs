// Generated macro for product_pack (macro)
macro_rules! Depcrate_product_tupleproduct_pack {
() => {
// Module: crate::product_tuple
// Provides: {"product_pack"}
// Dependencies: {}
macro_rules ! product_pack { ($ factor : expr) => { ($ factor ,) } ; ($ ($ factor : expr) ,*) => { ($ ($ factor) ,*) } ; ($ ($ factor : expr) ,*,) => { ($ ($ factor) ,*) } ; }
};
}
