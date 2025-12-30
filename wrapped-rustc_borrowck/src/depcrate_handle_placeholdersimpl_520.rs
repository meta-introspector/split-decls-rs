// Generated macro for impl_520 (impl)
macro_rules! Depcrate_handle_placeholdersimpl_520 {
() => {
// Module: crate::handle_placeholders
// Provides: {"impl_520"}
// Dependencies: {}
impl PlaceholderReachability { # [doc = " Merge the reachable placeholders of two graph components."] fn merge (self , other : PlaceholderReachability) -> PlaceholderReachability { use PlaceholderReachability :: * ; match (self , other) { (NoPlaceholders , NoPlaceholders) => NoPlaceholders , (NoPlaceholders , p @ Placeholders { .. }) | (p @ Placeholders { .. } , NoPlaceholders) => p , (Placeholders { min_placeholder : min_pl , max_placeholder : max_pl , max_universe : max_u , } , Placeholders { min_placeholder , max_placeholder , max_universe } ,) => Placeholders { min_placeholder : min_pl . min (min_placeholder) , max_placeholder : max_pl . max (max_placeholder) , max_universe : max_u . max (max_universe) , } , } } fn max_universe (& self) -> Option < (UniverseIndex , RegionVid) > { match self { Self :: NoPlaceholders => None , Self :: Placeholders { max_universe , .. } => Some (* max_universe) , } } # [doc = " If we have reached placeholders, determine if they can"] # [doc = " be named from this universe."] fn can_be_named_by (& self , from : UniverseIndex) -> bool { self . max_universe () . is_none_or (| (max_placeholder_universe , _) | from . can_name (max_placeholder_universe)) } }
};
}
