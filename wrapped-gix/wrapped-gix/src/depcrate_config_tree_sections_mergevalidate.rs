// Generated macro for validate (module)
macro_rules! Depcrate_config_tree_sections_mergevalidate {
() => {
// Module: crate::config::tree::sections::merge
// Provides: {"validate"}
// Dependencies: {}
# [cfg (feature = "merge")] mod validate { use crate :: { bstr :: BStr , config :: tree :: { keys , Merge } , } ; pub struct ConflictStyle ; impl keys :: Validate for ConflictStyle { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Merge :: CONFLICT_STYLE . try_into_conflict_style (value . into ()) ? ; Ok (()) } } }
};
}
