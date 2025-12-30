// Generated macro for impl_29 (impl)
macro_rules! Depcrate_manifestimpl_29 {
() => {
// Module: crate::manifest
// Provides: {"impl_29"}
// Dependencies: {}
impl Target { pub (crate) fn from_compressed_tar (builder : & mut Builder , base_path : & str) -> Self { let base_path = builder . input . join (base_path) ; let gz = tarball_variant (builder , & base_path , "gz") ; let xz = tarball_variant (builder , & base_path , "xz") ; if gz . is_none () && xz . is_none () { return Self :: unavailable () ; } Self { available : true , components : None , extensions : None , url : gz . as_ref () . map (| path | builder . url (path)) , hash : gz . map (FileHash :: Missing) , xz_url : xz . as_ref () . map (| path | builder . url (path)) , xz_hash : xz . map (FileHash :: Missing) , } } pub (crate) fn unavailable () -> Self { Self :: default () } }
};
}
