// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_tz_zicimpl_1147 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1147"}
// Dependencies: {}
impl FromStr for ZoneNameP { type Err = Error ; fn from_str (name : & str) -> Result < ZoneNameP , Error > { if name . is_empty () { return Err (err ! ("zone names cannot be empty")) ; } for component in name . split ('/') { if component == "." || component == ".." { return Err (err ! ("component {component:?} in zone name {name:?} cannot \
                     be \".\" or \"..\"" ,)) ; } } Ok (ZoneNameP { name : name . to_string () }) } }
};
}
