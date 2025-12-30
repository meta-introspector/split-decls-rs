// Generated macro for KeyEncodingType (enum)
macro_rules! Depcrate_db_optionsKeyEncodingType {
() => {
// Module: crate::db_options
// Provides: {"KeyEncodingType"}
// Dependencies: {}
# [doc = " Used in [`PlainTableFactoryOptions`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Default)] pub enum KeyEncodingType { # [doc = " Always write full keys."] # [default] Plain = 0 , # [doc = " Find opportunities to write the same prefix for multiple rows."] Prefix = 1 , }
};
}
