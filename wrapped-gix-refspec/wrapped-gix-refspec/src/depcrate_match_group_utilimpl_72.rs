// Generated macro for impl_72 (impl)
macro_rules! Depcrate_match_group_utilimpl_72 {
() => {
// Module: crate::match_group::util
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > From < & 'a BStr > for Needle < 'a > { fn from (v : & 'a BStr) -> Self { if let Some (pos) = v . find_byte (b'*') { Needle :: Glob { name : v , asterisk_pos : pos , } } else if v . starts_with (b"refs/") { Needle :: FullName (v) } else if let Ok (id) = gix_hash :: ObjectId :: from_hex (v) { Needle :: Object (id) } else { Needle :: PartialName (v) } } }
};
}
