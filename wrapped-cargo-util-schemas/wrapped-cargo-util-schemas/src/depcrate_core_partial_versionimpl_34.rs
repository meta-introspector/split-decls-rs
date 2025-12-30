// Generated macro for impl_34 (impl)
macro_rules! Depcrate_core_partial_versionimpl_34 {
() => {
// Module: crate::core::partial_version
// Provides: {"impl_34"}
// Dependencies: {}
impl std :: str :: FromStr for PartialVersion { type Err = PartialVersionError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { match semver :: Version :: parse (value) { Ok (ver) => Ok (ver . into ()) , Err (_) => { let mut version_req = match semver :: VersionReq :: parse (value) { Ok (req) => req , Err (_) if value . contains ('-') => return Err (ErrorKind :: Prerelease . into ()) , Err (_) if value . contains ('+') => return Err (ErrorKind :: BuildMetadata . into ()) , Err (_) => return Err (ErrorKind :: Unexpected . into ()) , } ; if version_req . comparators . len () != 1 { return Err (ErrorKind :: VersionReq . into ()) ; } let comp = version_req . comparators . pop () . unwrap () ; if comp . op != semver :: Op :: Caret { return Err (ErrorKind :: VersionReq . into ()) ; } else if value . starts_with ('^') { return Err (ErrorKind :: VersionReq . into ()) ; } let pre = if comp . pre . is_empty () { None } else { Some (comp . pre) } ; Ok (Self { major : comp . major , minor : comp . minor , patch : comp . patch , pre , build : None , }) } } } }
};
}
