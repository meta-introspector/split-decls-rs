// Generated macro for impl_1384 (impl)
macro_rules! Depcrate_zonedimpl_1384 {
() => {
// Module: crate::zoned
// Provides: {"impl_1384"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < std :: time :: SystemTime > for Zoned { type Error = Error ; # [inline] fn try_from (system_time : std :: time :: SystemTime) -> Result < Zoned , Error > { let timestamp = Timestamp :: try_from (system_time) ? ; Ok (Zoned :: new (timestamp , TimeZone :: system ())) } }
};
}
