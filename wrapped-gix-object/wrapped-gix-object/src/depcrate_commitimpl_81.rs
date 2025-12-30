// Generated macro for impl_81 (impl)
macro_rules! Depcrate_commitimpl_81 {
() => {
// Module: crate::commit
// Provides: {"impl_81"}
// Dependencies: {}
# [doc = " Instantiation and convenience."] impl < 'a , I > ExtraHeaders < I > where I : Iterator < Item = (& 'a BStr , & 'a BStr) > , { # [doc = " Create a new instance from an iterator over tuples of (name, value) pairs."] pub fn new (iter : I) -> Self { ExtraHeaders { inner : iter } } # [doc = " Find the _value_ of the _first_ header with the given `name`."] pub fn find (mut self , name : & str) -> Option < & 'a BStr > { self . inner . find_map (move | (k , v) | if k == name . as_bytes () . as_bstr () { Some (v) } else { None }) } # [doc = " Find the entry index with the given name, or return `None` if unavailable."] pub fn find_pos (self , name : & str) -> Option < usize > { self . inner . enumerate () . find_map (| (pos , (field , _value)) | (field == name) . then_some (pos)) } # [doc = " Return an iterator over all _values_ of headers with the given `name`."] pub fn find_all (self , name : & 'a str) -> impl Iterator < Item = & 'a BStr > { self . inner . filter_map (move | (k , v) | if k == name . as_bytes () . as_bstr () { Some (v) } else { None }) } # [doc = " Return an iterator over all git mergetags."] # [doc = ""] # [doc = " A merge tag is a tag object embedded within the respective header field of a commit, making"] # [doc = " it a child object of sorts."] pub fn mergetags (self) -> impl Iterator < Item = Result < TagRef < 'a > , crate :: decode :: Error > > { self . find_all ("mergetag") . map (| b | TagRef :: from_bytes (b)) } # [doc = " Return the cryptographic signature provided by gpg/pgp verbatim."] pub fn pgp_signature (self) -> Option < & 'a BStr > { self . find (SIGNATURE_FIELD_NAME) } }
};
}
