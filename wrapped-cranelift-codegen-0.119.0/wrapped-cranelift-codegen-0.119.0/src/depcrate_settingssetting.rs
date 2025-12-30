// Generated macro for Setting (struct)
macro_rules! Depcrate_settingsSetting {
() => {
// Module: crate::settings
// Provides: {"Setting"}
// Dependencies: {}
# [doc = " Represents an available builder setting."] # [doc = ""] # [doc = " This is used for iterating settings in a builder."] # [derive (Clone , Copy , Debug)] pub struct Setting { # [doc = " The name of the setting."] pub name : & 'static str , # [doc = " The description of the setting."] pub description : & 'static str , # [doc = " The kind of the setting."] pub kind : SettingKind , # [doc = " The supported values of the setting (for enum values)."] pub values : Option < & 'static [& 'static str] > , }
};
}
