// Generated macro for Location (struct)
macro_rules! Depcrate_astronomyLocation {
() => {
// Module: crate::astronomy
// Provides: {"Location"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq)] # [doc = " A Location on the Earth given as a latitude, longitude, elevation, and standard time zone."] # [doc = " Latitude is given in degrees from -90 to 90, longitude in degrees from -180 to 180,"] # [doc = " elevation in meters, and zone as a UTC offset in fractional days (ex. UTC+1 would have zone = 1.0 / 24.0)"] # [allow (clippy :: exhaustive_structs)] pub struct Location { # [doc = " latitude from -90 to 90"] pub (crate) latitude : f64 , # [doc = " longitude from -180 to 180"] pub (crate) longitude : f64 , # [doc = " elevation in meters"] pub (crate) elevation : f64 , # [doc = " UTC timezone offset in fractional days (1 hr = 1.0 / 24.0 day)"] pub (crate) utc_offset : f64 , }
};
}
