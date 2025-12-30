// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_tz_tzifimpl_1094 {
() => {
// Module: crate::tz::tzif
// Provides: {"impl_1094"}
// Dependencies: {}
impl < STR : AsRef < str > , ABBREV , TYPES , TIMESTAMPS , STARTS , ENDS , INFOS > PartialEq for Tzif < STR , ABBREV , TYPES , TIMESTAMPS , STARTS , ENDS , INFOS > { fn eq (& self , rhs : & Self) -> bool { self . inner . fixed . name . as_ref () . map (| n | n . as_ref ()) == rhs . inner . fixed . name . as_ref () . map (| n | n . as_ref ()) && self . inner . fixed . checksum == rhs . inner . fixed . checksum } }
};
}
