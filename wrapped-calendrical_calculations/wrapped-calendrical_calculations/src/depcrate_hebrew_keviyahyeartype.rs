// Generated macro for YearType (enum)
macro_rules! Depcrate_hebrew_keviyahYearType {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"YearType"}
// Dependencies: {}
# [doc = " The type of year it is"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] # [allow (clippy :: exhaustive_enums)] pub enum YearType { # [doc = " חסרה: both Ḥeshvan and Kislev have 29 days"] Deficient = - 1 , # [doc = " כסדרה: Ḥeshvan has 29, Kislev has 30"] Regular = 0 , # [doc = " שלמה: both Ḥeshvan and Kislev have 30 days"] Complete = 1 , }
};
}
