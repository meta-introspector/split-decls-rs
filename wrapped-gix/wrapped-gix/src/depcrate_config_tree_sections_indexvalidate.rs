// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_indexvalidate {
() => {
// Module: crate::config::tree::sections::index
// Provides: {"validate"}
// Dependencies: {}
mod validate { use crate :: { bstr :: BStr , config :: tree :: keys } ; pub struct IndexThreads ; impl keys :: Validate for IndexThreads { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { super :: Index :: THREADS . try_into_index_threads (value . into ()) ? ; Ok (()) } } }
};
}
