// Generated macro for AttrSourceMap (struct)
macro_rules! Depcrate_attrAttrSourceMap {
() => {
// Module: crate::attr
// Provides: {"AttrSourceMap"}
// Dependencies: {}
# [derive (Debug)] pub struct AttrSourceMap { source : Vec < Either < ast :: Attr , ast :: Comment > > , file_id : HirFileId , # [doc = " If this map is for a module, this will be the [`HirFileId`] of the module's definition site,"] # [doc = " while `file_id` will be the one of the module declaration site."] # [doc = " The usize is the index into `source` from which point on the entries reside in the def site"] # [doc = " file."] mod_def_site_file_id : Option < (HirFileId , usize) > , }
};
}
