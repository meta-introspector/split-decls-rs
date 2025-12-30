// Generated macro for get_trait_info_from_bound (function)
macro_rules! Depcrate_trait_boundsget_trait_info_from_bound {
() => {
// Module: crate::trait_bounds
// Provides: {"get_trait_info_from_bound"}
// Dependencies: {}
fn get_trait_info_from_bound < 'a > (bound : & 'a GenericBound < '_ >) -> Option < (Res , & 'a [PathSegment < 'a >] , Span) > { if let GenericBound :: Trait (t) = bound { let trait_path = t . trait_ref . path ; Some ((trait_path . res , trait_path . segments , t . span)) } else { None } }
};
}
