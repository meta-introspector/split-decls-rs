// Generated macro for impl_88 (impl)
macro_rules! Depcrate_object_convertimpl_88 {
() => {
// Module: crate::object::convert
// Provides: {"impl_88"}
// Dependencies: {}
impl TryFrom < TagRef < '_ > > for Tag { type Error = crate :: decode :: Error ; fn try_from (other : TagRef < '_ >) -> Result < Tag , Self :: Error > { let TagRef { target , name , target_kind , message , tagger , pgp_signature , } = other ; let untrimmed_tagger = tagger . map (parse_signature) . transpose () ? . map (Into :: into) ; Ok (Tag { target : gix_hash :: ObjectId :: from_hex (target) . expect ("prior parser validation") , name : name . to_owned () , target_kind , message : message . to_owned () , tagger : untrimmed_tagger , pgp_signature : pgp_signature . map (ToOwned :: to_owned) , }) } }
};
}
