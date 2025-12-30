// Generated macro for Sign (enum)
macro_rules! Depcrate_variationsSign {
() => {
// Module: crate::variations
// Provides: {"Sign"}
// Dependencies: {}
# [doc = " A specification of the sign used when formatting a number."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Default)] # [allow (clippy :: exhaustive_enums)] pub enum Sign { # [doc = " No sign (implicitly positive, e.g., 1729)."] # [default] None , # [doc = " A negative sign, e.g., -1729."] Negative , # [doc = " An explicit positive sign, e.g., +1729."] Positive , }
};
}
