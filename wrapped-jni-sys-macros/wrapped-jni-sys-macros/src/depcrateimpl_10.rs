// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl syn :: parse :: Parse for JniVersion { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { let version : LitStr = input . parse () ? ; let version = version . value () ; if version == "reserved" { return Ok (JniVersion { major : 999 , minor : 0 , }) ; } let mut split = version . splitn (2 , '.') ; const EXPECTED_MSG : & str = "Expected \"major.minor\" version number or \"reserved\"" ; let major = split . next () . ok_or (syn :: Error :: new (input . span () , EXPECTED_MSG)) ? ; let major = major . parse :: < u16 > () . map_err (| _ | syn :: Error :: new (input . span () , EXPECTED_MSG)) ? ; let minor = split . next () . unwrap_or ("0") . parse :: < u16 > () . map_err (| _ | syn :: Error :: new (input . span () , EXPECTED_MSG)) ? ; Ok (JniVersion { major , minor }) } }
};
}
