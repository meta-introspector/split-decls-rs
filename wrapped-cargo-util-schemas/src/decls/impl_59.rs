macro_rules! deps {
    () => {
        Result!();
        TomlLockfilePackageIdError!();
        TomlLockfileSourceId!();
        TomlLockfilePackageIdErrorKind!();
        TomlLockfilePackageId!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl FromStr for TomlLockfilePackageId { type Err = TomlLockfilePackageIdError ; fn from_str (s : & str) -> Result < TomlLockfilePackageId , Self :: Err > { let mut s = s . splitn (3 , ' ') ; let name = s . next () . unwrap () ; let version = s . next () ; let source_id = match s . next () { Some (s) => { if let Some (s) = s . strip_prefix ('(') . and_then (| s | s . strip_suffix (')')) { Some (TomlLockfileSourceId :: new (s . to_string ()) ?) } else { return Err (TomlLockfilePackageIdErrorKind :: InvalidSerializedPackageId . into ()) ; } } None => None , } ; Ok (TomlLockfilePackageId { name : name . to_string () , version : version . map (| v | v . to_string ()) , source : source_id , }) } }
    };
}

impl_59!()