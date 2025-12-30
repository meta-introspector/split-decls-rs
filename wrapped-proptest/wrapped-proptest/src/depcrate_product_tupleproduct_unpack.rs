// Generated macro for product_unpack (macro)
macro_rules! Depcrate_product_tupleproduct_unpack {
() => {
// Module: crate::product_tuple
// Provides: {"product_unpack"}
// Dependencies: {}
macro_rules ! product_unpack { ($ factor : pat) => { ($ factor ,) } ; ($ ($ factor : pat) ,*) => { ($ ($ factor) ,*) } ; ($ ($ factor : pat) ,*,) => { ($ ($ factor) ,*) } ; }
};
}
