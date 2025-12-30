// Generated macro for pick_path_for_usages (function)
macro_rules! Depcrate_searchpick_path_for_usages {
() => {
// Module: crate::search
// Provides: {"pick_path_for_usages"}
// Dependencies: {}
# [doc = " Returns a path that's suitable for path resolution. We exclude builtin types, since they aren't"] # [doc = " something that we can find references to. We then somewhat arbitrarily pick the path that is the"] # [doc = " longest as this is hopefully more likely to be less common, making it faster to find."] fn pick_path_for_usages < 'a > (pattern : & 'a ResolvedPattern < '_ >) -> Option < & 'a ResolvedPath > { pattern . resolved_paths . iter () . filter (| (_ , p) | { ! matches ! (p . resolution , hir :: PathResolution :: Def (hir :: ModuleDef :: BuiltinType (_))) }) . map (| (node , resolved) | (node . text () . len () , resolved)) . max_by (| (a , _) , (b , _) | a . cmp (b)) . map (| (_ , resolved) | resolved) }
};
}
