// Generated macro for macro_96 (macro)
macro_rules! Depcrate_eventmacro_96 {
() => {
// Module: crate::event
// Provides: {"macro_96"}
// Dependencies: {}
bitflags ! { # [doc = " Represents key modifiers (shift, control, alt, etc.)."] # [doc = ""] # [doc = " **Note:** `SUPER`, `HYPER`, and `META` can only be read if"] # [doc = " [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with"] # [doc = " [`PushKeyboardEnhancementFlags`]."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (transparent))] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] pub struct KeyModifiers : u8 { const SHIFT = 0b0000_0001 ; const CONTROL = 0b0000_0010 ; const ALT = 0b0000_0100 ; const SUPER = 0b0000_1000 ; const HYPER = 0b0001_0000 ; const META = 0b0010_0000 ; const NONE = 0b0000_0000 ; } }
};
}
