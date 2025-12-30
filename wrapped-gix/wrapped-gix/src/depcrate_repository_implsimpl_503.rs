// Generated macro for impl_503 (impl)
macro_rules! Depcrate_repository_implsimpl_503 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_503"}
// Dependencies: {}
impl gix_object :: Find for crate :: Repository { fn try_find < 'a > (& self , id : & gix_hash :: oid , buffer : & 'a mut Vec < u8 > ,) -> Result < Option < gix_object :: Data < 'a > > , gix_object :: find :: Error > { if id == ObjectId :: empty_tree (self . object_hash ()) { buffer . clear () ; return Ok (Some (gix_object :: Data { kind : gix_object :: Kind :: Tree , data : & [] , })) ; } self . objects . try_find (id , buffer) } }
};
}
