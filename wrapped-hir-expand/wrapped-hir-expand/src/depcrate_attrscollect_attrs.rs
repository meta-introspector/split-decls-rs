// Generated macro for collect_attrs (function)
macro_rules! Depcrate_attrscollect_attrs {
() => {
// Module: crate::attrs
// Provides: {"collect_attrs"}
// Dependencies: {}
pub fn collect_attrs (owner : & dyn ast :: HasAttrs ,) -> impl Iterator < Item = (AttrId , Either < ast :: Attr , ast :: Comment >) > { let inner_attrs = inner_attributes (owner . syntax ()) . into_iter () . flatten () . zip (iter :: repeat (true)) ; let outer_attrs = ast :: AttrDocCommentIter :: from_syntax_node (owner . syntax ()) . filter (| el | match el { Either :: Left (attr) => attr . kind () . is_outer () , Either :: Right (comment) => comment . is_outer () , }) . zip (iter :: repeat (false)) ; outer_attrs . chain (inner_attrs) . enumerate () . map (| (id , (attr , is_inner)) | (AttrId :: new (id , is_inner) , attr)) }
};
}
