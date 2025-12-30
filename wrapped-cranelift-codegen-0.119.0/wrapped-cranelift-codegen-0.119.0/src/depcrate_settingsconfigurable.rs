// Generated macro for Configurable (trait)
macro_rules! Depcrate_settingsConfigurable {
() => {
// Module: crate::settings
// Provides: {"Configurable"}
// Dependencies: {}
# [doc = " A string-based configurator for settings groups."] # [doc = ""] # [doc = " The `Configurable` protocol allows settings to be modified by name before a finished `Flags`"] # [doc = " struct is created."] pub trait Configurable { # [doc = " Set the string value of any setting by name."] # [doc = ""] # [doc = " This can set any type of setting whether it is numeric, boolean, or enumerated."] fn set (& mut self , name : & str , value : & str) -> SetResult < () > ; # [doc = " Enable a boolean setting or apply a preset."] # [doc = ""] # [doc = " If the identified setting isn't a boolean or a preset, a `BadType` error is returned."] fn enable (& mut self , name : & str) -> SetResult < () > ; }
};
}
