macro_rules! deps {
    () => {
        ConfigLevel!();
        ConfigEntry!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'cfg > ConfigEntry < 'cfg > { # [doc = " Gets the name of this entry."] # [doc = ""] # [doc = " May return `None` if the name is not valid utf-8"] pub fn name (& self) -> Option < & str > { str :: from_utf8 (self . name_bytes ()) . ok () } # [doc = " Gets the name of this entry as a byte slice."] pub fn name_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , (* self . raw) . name) . unwrap () } } # [doc = " Gets the value of this entry."] # [doc = ""] # [doc = " May return `None` if the value is not valid utf-8"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics when no value is defined."] pub fn value (& self) -> Option < & str > { str :: from_utf8 (self . value_bytes ()) . ok () } # [doc = " Gets the value of this entry as a byte slice."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics when no value is defined."] pub fn value_bytes (& self) -> & [u8] { unsafe { crate :: opt_bytes (self , (* self . raw) . value) . unwrap () } } # [doc = " Returns `true` when a value is defined otherwise `false`."] # [doc = ""] # [doc = " No value defined is a short-hand to represent a Boolean `true`."] pub fn has_value (& self) -> bool { unsafe { ! (* self . raw) . value . is_null () } } # [doc = " Gets the configuration level of this entry."] pub fn level (& self) -> ConfigLevel { unsafe { ConfigLevel :: from_raw ((* self . raw) . level) } } # [doc = " Depth of includes where this variable was found"] pub fn include_depth (& self) -> u32 { unsafe { (* self . raw) . include_depth as u32 } } }
    };
}

impl_271!();