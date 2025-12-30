// Generated macro for DateWithYear (enum)
macro_rules! Depcrate_civil_dateDateWithYear {
() => {
// Module: crate::civil::date
// Provides: {"DateWithYear"}
// Dependencies: {}
# [doc = " Encodes the \"with year\" option of [`DateWith`]."] # [doc = ""] # [doc = " This encodes the invariant that `DateWith::year` and `DateWith::era_year`"] # [doc = " are mutually exclusive and override each other."] # [derive (Clone , Copy , Debug)] enum DateWithYear { Jiff (i16) , EraYear (i16 , Era) , }
};
}
