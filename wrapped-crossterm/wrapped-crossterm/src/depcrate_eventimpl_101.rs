// Generated macro for impl_101 (impl)
macro_rules! Depcrate_eventimpl_101 {
() => {
// Module: crate::event
// Provides: {"impl_101"}
// Dependencies: {}
impl KeyEvent { pub const fn new (code : KeyCode , modifiers : KeyModifiers) -> KeyEvent { KeyEvent { code , modifiers , kind : KeyEventKind :: Press , state : KeyEventState :: empty () , } } pub const fn new_with_kind (code : KeyCode , modifiers : KeyModifiers , kind : KeyEventKind ,) -> KeyEvent { KeyEvent { code , modifiers , kind , state : KeyEventState :: empty () , } } pub const fn new_with_kind_and_state (code : KeyCode , modifiers : KeyModifiers , kind : KeyEventKind , state : KeyEventState ,) -> KeyEvent { KeyEvent { code , modifiers , kind , state , } } fn normalize_case (mut self) -> KeyEvent { let c = match self . code { KeyCode :: Char (c) => c , _ => return self , } ; if c . is_ascii_uppercase () { self . modifiers . insert (KeyModifiers :: SHIFT) ; } else if self . modifiers . contains (KeyModifiers :: SHIFT) { self . code = KeyCode :: Char (c . to_ascii_uppercase ()) } self } # [doc = " Returns whether the key event is a press event."] pub fn is_press (& self) -> bool { matches ! (self . kind , KeyEventKind :: Press) } # [doc = " Returns whether the key event is a release event."] pub fn is_release (& self) -> bool { matches ! (self . kind , KeyEventKind :: Release) } # [doc = " Returns whether the key event is a repeat event."] pub fn is_repeat (& self) -> bool { matches ! (self . kind , KeyEventKind :: Repeat) } }
};
}
