// Generated macro for KeyEvent (struct)
macro_rules! Depcrate_eventKeyEvent {
() => {
// Module: crate::event
// Provides: {"KeyEvent"}
// Dependencies: {}
# [doc = " Represents a key event."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Debug , PartialOrd , Ord , Clone , Copy)] pub struct KeyEvent { # [doc = " The key itself."] pub code : KeyCode , # [doc = " Additional key modifiers."] pub modifiers : KeyModifiers , # [doc = " Kind of event."] # [doc = ""] # [doc = " Only set if:"] # [doc = " - Unix: [`KeyboardEnhancementFlags::REPORT_EVENT_TYPES`] has been enabled with [`PushKeyboardEnhancementFlags`]."] # [doc = " - Windows: always"] pub kind : KeyEventKind , # [doc = " Keyboard state."] # [doc = ""] # [doc = " Only set if [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with"] # [doc = " [`PushKeyboardEnhancementFlags`]."] pub state : KeyEventState , }
};
}
