// Generated macro for impl_91 (impl)
macro_rules! Depcrate_lockfileimpl_91 {
() => {
// Module: crate::lockfile
// Provides: {"impl_91"}
// Dependencies: {}
impl FromStr for TomlLockfilePackageId { type Err = TomlLockfilePackageIdError ; fn from_str (s : & str) -> Result < TomlLockfilePackageId , Self :: Err > { let mut s = s . splitn (3 , ' ') ; let name = s . next () . unwrap () ; let version = s . next () ; let source_id = match s . next () { Some (s) => { if let Some (s) = s . strip_prefix ('(') . and_then (| s | s . strip_suffix (')')) { Some (TomlLockfileSourceId :: new (s . to_string ()) ?) } else { return Err (TomlLockfilePackageIdErrorKind :: InvalidSerializedPackageId . into ()) ; } } None => None , } ; Ok (TomlLockfilePackageId { name : name . to_string () , version : version . map (| v | v . to_string ()) , source : source_id , }) } }
};
}
