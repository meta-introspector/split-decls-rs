// Generated macro for parse_spec (function)
macro_rules! Depcrate_core_package_id_specparse_spec {
() => {
// Module: crate::core::package_id_spec
// Provides: {"parse_spec"}
// Dependencies: {}
fn parse_spec (spec : & str) -> Result < Option < (String , Option < PartialVersion >) > > { let Some ((name , ver)) = spec . rsplit_once ('@') . or_else (| | spec . rsplit_once (':') . filter (| (n , _) | ! n . ends_with (':'))) else { return Ok (None) ; } ; let name = name . to_owned () ; let ver = ver . parse :: < PartialVersion > () ? ; Ok (Some ((name , Some (ver)))) }
};
}
