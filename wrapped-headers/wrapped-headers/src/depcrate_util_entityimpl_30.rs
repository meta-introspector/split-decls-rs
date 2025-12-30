// Generated macro for impl_30 (impl)
macro_rules! Depcrate_util_entityimpl_30 {
() => {
// Module: crate::util::entity
// Provides: {"impl_30"}
// Dependencies: {}
impl EntityTag { # [cfg (test)] pub fn from_static (bytes : & 'static str) -> EntityTag { let val = HeaderValue :: from_static (bytes) ; match EntityTag :: from_val (& val) { Some (tag) => tag , None => { panic ! ("invalid static string for EntityTag: {:?}" , bytes) ; } } } pub (crate) fn from_owned (val : HeaderValue) -> Option < EntityTag > { EntityTag :: parse (val . as_bytes ()) ? ; Some (EntityTag (val)) } pub (crate) fn from_val (val : & HeaderValue) -> Option < EntityTag > { EntityTag :: parse (val . as_bytes ()) . map (| _entity | EntityTag (val . clone ())) } }
};
}
