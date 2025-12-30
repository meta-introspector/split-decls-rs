// Generated macro for impl_2760 (impl)
macro_rules! Depcrate_settingsimpl_2760 {
() => {
// Module: crate::settings
// Provides: {"impl_2760"}
// Dependencies: {}
impl Value { # [doc = " Gets the kind of setting."] pub fn kind (& self) -> SettingKind { match & self . detail { detail :: Detail :: Enum { .. } => SettingKind :: Enum , detail :: Detail :: Num => SettingKind :: Num , detail :: Detail :: Bool { .. } => SettingKind :: Bool , detail :: Detail :: Preset => unreachable ! () , } } # [doc = " Gets the enum value if the value is from an enum setting."] pub fn as_enum (& self) -> Option < & 'static str > { self . values . map (| v | v [self . value as usize]) } # [doc = " Gets the numerical value if the value is from a num setting."] pub fn as_num (& self) -> Option < u8 > { match & self . detail { detail :: Detail :: Num => Some (self . value) , _ => None , } } # [doc = " Gets the boolean value if the value is from a boolean setting."] pub fn as_bool (& self) -> Option < bool > { match & self . detail { detail :: Detail :: Bool { bit } => Some (self . value & (1 << bit) != 0) , _ => None , } } # [doc = " Builds a string from the current value"] pub fn value_string (& self) -> String { match self . kind () { SettingKind :: Enum => self . as_enum () . map (| b | b . to_string ()) , SettingKind :: Num => self . as_num () . map (| b | b . to_string ()) , SettingKind :: Bool => self . as_bool () . map (| b | b . to_string ()) , SettingKind :: Preset => unreachable ! () , } . unwrap () } }
};
}
