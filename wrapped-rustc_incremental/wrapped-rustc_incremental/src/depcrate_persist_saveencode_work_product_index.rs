// Generated macro for encode_work_product_index (function)
macro_rules! Depcrate_persist_saveencode_work_product_index {
() => {
// Module: crate::persist::save
// Provides: {"encode_work_product_index"}
// Dependencies: {}
fn encode_work_product_index (work_products : & FxIndexMap < WorkProductId , WorkProduct > , encoder : & mut FileEncoder ,) { let serialized_products : Vec < _ > = work_products . iter () . map (| (id , work_product) | SerializedWorkProduct { id : * id , work_product : work_product . clone () , }) . collect () ; serialized_products . encode (encoder) }
};
}
