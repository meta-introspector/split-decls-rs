// Generated macro for YearInfo (struct)
macro_rules! Depcrate_hebrew_keviyahYearInfo {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"YearInfo"}
// Dependencies: {}
# [doc = " Everything about a given year. Can be conveniently packed down into an i64 if needed."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] # [allow (clippy :: exhaustive_structs)] pub struct YearInfo { # [doc = " The Keviyah of the year"] pub keviyah : Keviyah , # [doc = " How many full weeks have passed since the week of Beharad"] pub weeks_since_beharad : i64 , }
};
}
