// Generated macro for impl_26 (impl)
macro_rules! Depcrate_manifestimpl_26 {
() => {
// Module: crate::manifest
// Provides: {"impl_26"}
// Dependencies: {}
impl Artifact { pub (crate) fn add_file (& mut self , builder : & mut Builder , target : & str , path : & str) { if let Some (path) = record_shipped_file (builder , builder . input . join (path)) { self . target . entry (target . into ()) . or_insert_with (Vec :: new) . push (ArtifactFile { url : builder . url (& path) , hash_sha256 : FileHash :: Missing (path) , }) ; } } pub (crate) fn add_tarball (& mut self , builder : & mut Builder , target : & str , base_path : & str) { let files = self . target . entry (target . into ()) . or_insert_with (Vec :: new) ; let base_path = builder . input . join (base_path) ; for compression in & ["gz" , "xz"] { if let Some (tarball) = tarball_variant (builder , & base_path , compression) { files . push (ArtifactFile { url : builder . url (& tarball) , hash_sha256 : FileHash :: Missing (tarball) , }) ; } } } }
};
}
