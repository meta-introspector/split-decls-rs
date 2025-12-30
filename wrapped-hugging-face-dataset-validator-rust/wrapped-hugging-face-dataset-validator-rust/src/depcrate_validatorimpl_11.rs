// Generated macro for impl_11 (impl)
macro_rules! Depcrate_validatorimpl_11 {
() => {
// Module: crate::validator
// Provides: {"impl_11"}
// Dependencies: {}
impl EntityIdentifier { pub fn new_dataset (dataset : String) -> Self { Self { dataset , config : None , split : None , } } pub fn new_config (dataset : String , config : String) -> Self { Self { dataset , config : Some (config) , split : None , } } pub fn new_split (dataset : String , config : String , split : String) -> Self { Self { dataset , config : Some (config) , split : Some (split) , } } pub fn infer_level (& self) -> ValidationLevel { match (& self . config , & self . split) { (Some (_) , Some (_)) => ValidationLevel :: Split , (Some (_) , None) => ValidationLevel :: Config , (None , None) => ValidationLevel :: Dataset , (None , Some (_)) => ValidationLevel :: Dataset , } } pub fn cache_key (& self , kind : & str) -> String { format ! ("{}:{}:{}:{}" , kind , self . dataset , self . config . as_deref () . unwrap_or ("") , self . split . as_deref () . unwrap_or ("")) } }
};
}
