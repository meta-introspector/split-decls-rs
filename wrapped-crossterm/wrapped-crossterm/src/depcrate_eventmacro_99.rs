// Generated macro for macro_99 (macro)
macro_rules! Depcrate_eventmacro_99 {
() => {
// Module: crate::event
// Provides: {"macro_99"}
// Dependencies: {}
bitflags ! { # [doc = " Represents extra state about the key event."] # [doc = ""] # [doc = " **Note:** This state can only be read if"] # [doc = " [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with"] # [doc = " [`PushKeyboardEnhancementFlags`]."] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (transparent))] pub struct KeyEventState : u8 { # [doc = " The key event origins from the keypad."] const KEYPAD = 0b0000_0001 ; # [doc = " Caps Lock was enabled for this key event."] # [doc = ""] # [doc = " **Note:** this is set for the initial press of Caps Lock itself."] const CAPS_LOCK = 0b0000_0010 ; # [doc = " Num Lock was enabled for this key event."] # [doc = ""] # [doc = " **Note:** this is set for the initial press of Num Lock itself."] const NUM_LOCK = 0b0000_0100 ; const NONE = 0b0000_0000 ; } }
};
}
