// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_tz_zicimpl_1122 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1122"}
// Dependencies: {}
impl LinkP { fn parse (fields : & [& str]) -> Result < LinkP , Error > { if fields . len () != 2 { return Err (err ! ("expected exactly 2 fields after LINK, but found {}" , fields . len ())) ; } let target = fields [0] . parse :: < ZoneNameP > () . map_err (| e | e . context ("failed to parse LINK target")) ? ; let name = fields [1] . parse :: < ZoneNameP > () . map_err (| e | e . context ("failed to parse LINK name")) ? ; Ok (LinkP { target , name }) } }
};
}
