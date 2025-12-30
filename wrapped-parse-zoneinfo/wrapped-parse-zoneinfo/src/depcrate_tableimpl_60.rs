// Generated macro for impl_60 (impl)
macro_rules! Depcrate_tableimpl_60 {
() => {
// Module: crate::table
// Provides: {"impl_60"}
// Dependencies: {}
impl Table { # [doc = " Tries to find the zoneset with the given name by looking it up in"] # [doc = " either the zonesets map or the links map."] pub fn get_zoneset (& self , zone_name : & str) -> Option < & [ZoneInfo] > { if self . zonesets . contains_key (zone_name) { Some (& * self . zonesets [zone_name]) } else if self . links . contains_key (zone_name) { let target = & self . links [zone_name] ; Some (& * self . zonesets [target]) } else { None } } }
};
}
