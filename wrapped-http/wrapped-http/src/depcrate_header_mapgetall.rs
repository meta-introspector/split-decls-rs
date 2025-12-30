// Generated macro for GetAll (struct)
macro_rules! Depcrate_header_mapGetAll {
() => {
// Module: crate::header::map
// Provides: {"GetAll"}
// Dependencies: {}
# [doc = " A view to all values stored in a single entry."] # [doc = ""] # [doc = " This struct is returned by `HeaderMap::get_all`."] # [derive (Debug)] pub struct GetAll < 'a , T > { map : & 'a HeaderMap < T > , index : Option < usize > , }
};
}
