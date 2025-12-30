// Generated macro for AgeField (struct)
macro_rules! DepcrateAgeField {
() => {
// Module: crate
// Provides: {"AgeField"}
// Dependencies: {}
# [doc = " A new-type representing a person's age in years (0-130)."] # [derive (Default , Clone , Copy , Serialize)] struct AgeField { # [serde (skip)] label : & 'static str , value : u8 , }
};
}
