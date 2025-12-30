// Generated macro for macro_74 (macro)
macro_rules! Depcrate_eventmacro_74 {
() => {
// Module: crate::event
// Provides: {"macro_74"}
// Dependencies: {}
bitflags ! { # [doc = " Represents special flags that tell compatible terminals to add extra information to keyboard events."] # [doc = ""] # [doc = " See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement> for more information."] # [doc = ""] # [doc = " Alternate keys and Unicode codepoints are not yet supported by crossterm."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (transparent))] # [derive (Debug , PartialOrd , PartialEq , Eq , Clone , Copy , Hash)] pub struct KeyboardEnhancementFlags : u8 { # [doc = " Represent Escape and modified keys using CSI-u sequences, so they can be unambiguously"] # [doc = " read."] const DISAMBIGUATE_ESCAPE_CODES = 0b0000_0001 ; # [doc = " Add extra events with [`KeyEvent.kind`] set to [`KeyEventKind::Repeat`] or"] # [doc = " [`KeyEventKind::Release`] when keys are autorepeated or released."] const REPORT_EVENT_TYPES = 0b0000_0010 ; # [doc = " Send [alternate keycodes](https://sw.kovidgoyal.net/kitty/keyboard-protocol/#key-codes)"] # [doc = " in addition to the base keycode. The alternate keycode overrides the base keycode in"] # [doc = " resulting `KeyEvent`s."] const REPORT_ALTERNATE_KEYS = 0b0000_0100 ; # [doc = " Represent all keyboard events as CSI-u sequences. This is required to get repeat/release"] # [doc = " events for plain-text keys."] const REPORT_ALL_KEYS_AS_ESCAPE_CODES = 0b0000_1000 ; } }
};
}
