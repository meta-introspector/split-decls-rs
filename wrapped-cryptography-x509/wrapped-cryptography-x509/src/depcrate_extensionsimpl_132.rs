// Generated macro for impl_132 (impl)
macro_rules! Depcrate_extensionsimpl_132 {
() => {
// Module: crate::extensions
// Provides: {"impl_132"}
// Dependencies: {}
impl < 'a > Extensions < 'a > { # [doc = " Create an `Extensions` from the given `RawExtensions`."] # [doc = ""] # [doc = " Returns an `Err` variant containing the first duplicated extension's"] # [doc = " OID, if there are any duplicates."] pub fn from_raw_extensions (raw : Option < & RawExtensions < 'a > > ,) -> Result < Self , DuplicateExtensionsError > { match raw { Some (raw_exts) => { let mut seen_oids = HashSet :: new () ; for ext in raw_exts . unwrap_read () . clone () { if ! seen_oids . insert (ext . extn_id . clone ()) { return Err (DuplicateExtensionsError (ext . extn_id)) ; } } Ok (Self (Some (raw_exts . clone ()))) } None => Ok (Self (None)) , } } # [doc = " Retrieves the extension identified by the given OID,"] # [doc = " or None if the extension is not present (or no extensions are present)."] pub fn get_extension (& self , oid : & asn1 :: ObjectIdentifier) -> Option < Extension < 'a > > { self . iter () . find (| ext | & ext . extn_id == oid) } # [doc = " Returns a reference to the underlying extensions."] pub fn as_raw (& self) -> Option < & RawExtensions < 'a > > { self . 0 . as_ref () } # [doc = " Returns an iterator over the underlying extensions."] pub fn iter (& self) -> impl Iterator < Item = Extension < 'a > > { self . as_raw () . map (| raw | raw . unwrap_read () . clone ()) . into_iter () . flatten () } }
};
}
