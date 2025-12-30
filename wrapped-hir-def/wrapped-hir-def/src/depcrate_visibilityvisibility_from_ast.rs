// Generated macro for visibility_from_ast (function)
macro_rules! Depcrate_visibilityvisibility_from_ast {
() => {
// Module: crate::visibility
// Provides: {"visibility_from_ast"}
// Dependencies: {}
pub fn visibility_from_ast (db : & dyn DefDatabase , has_resolver : impl HasResolver + HasModule , ast_vis : InFile < Option < ast :: Visibility > > ,) -> Visibility { let mut span_map = None ; let raw_vis = crate :: item_tree :: visibility_from_ast (db , ast_vis . value , & mut | range | { span_map . get_or_insert_with (| | db . span_map (ast_vis . file_id)) . span_for_range (range) . ctx }) ; match raw_vis { RawVisibility :: PubSelf (explicitness) => { Visibility :: Module (has_resolver . module (db) , explicitness) } RawVisibility :: PubCrate => Visibility :: PubCrate (has_resolver . krate (db)) , RawVisibility :: Public => Visibility :: Public , RawVisibility :: Module (..) => Visibility :: resolve (db , & has_resolver . resolver (db) , & raw_vis) , } }
};
}
