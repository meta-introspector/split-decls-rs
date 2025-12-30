// Generated macro for SerializedWorkProduct (struct)
macro_rules! Depcrate_persist_dataSerializedWorkProduct {
() => {
// Module: crate::persist::data
// Provides: {"SerializedWorkProduct"}
// Dependencies: {}
# [derive (Debug , Encodable , Decodable)] pub (crate) struct SerializedWorkProduct { # [doc = " node that produced the work-product"] pub id : WorkProductId , # [doc = " work-product data itself"] pub work_product : WorkProduct , }
};
}
