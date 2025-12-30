// Generated macro for impl_86 (impl)
macro_rules! Depcrate_h2impl_86 {
() => {
// Module: crate::h2
// Provides: {"impl_86"}
// Dependencies: {}
impl TryFrom < & [String] > for Http2Settings { type Error = String ; fn try_from (settings : & [String]) -> Result < Self , Self :: Error > { let re = Regex :: new (H2_SEND_SETTINGS_PATTERN) . unwrap () ; let mut parsed = Self :: default () ; for setting in settings { match re . captures (setting) { Some (caps) => match (caps . get (1) , caps . get (2)) { (Some (id) , Some (value)) => { match (id . as_str () . parse :: < u16 > () , value . as_str () . parse :: < u32 > () ,) { (Ok (id) , Ok (v)) => { parsed . set_from_wire (id , v) ; } , _ => return Err (format ! ("error: parsing H2 setting {}" , setting)) , } } , _ => return Err (format ! ("error: parsing H2 setting {}" , setting)) , } , None => return Err (format ! ("error: parsing H2 setting {}" , setting)) , } } Ok (parsed) } }
};
}
