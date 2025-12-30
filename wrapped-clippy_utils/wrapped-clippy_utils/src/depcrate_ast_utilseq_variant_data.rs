// Generated macro for eq_variant_data (function)
macro_rules! Depcrate_ast_utilseq_variant_data {
() => {
// Module: crate::ast_utils
// Provides: {"eq_variant_data"}
// Dependencies: {}
pub fn eq_variant_data (l : & VariantData , r : & VariantData) -> bool { use VariantData :: * ; match (l , r) { (Unit (_) , Unit (_)) => true , (Struct { fields : l , .. } , Struct { fields : r , .. }) | (Tuple (l , _) , Tuple (r , _)) => { over (l , r , eq_struct_field) } , _ => false , } }
};
}
