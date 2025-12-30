// Generated macro for Value (struct)
macro_rules! Depcrate_settingsValue {
() => {
// Module: crate::settings
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Represents a setting value."] # [doc = ""] # [doc = " This is used for iterating values in `Flags`."] pub struct Value { # [doc = " The name of the setting associated with this value."] pub name : & 'static str , pub (crate) detail : detail :: Detail , pub (crate) values : Option < & 'static [& 'static str] > , pub (crate) value : u8 , }
};
}
