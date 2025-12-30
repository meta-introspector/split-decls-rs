// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl AgeField { const MAX : u8 = 130 ; const fn new (label : & 'static str) -> Self { Self { label , value : 0 } } # [doc = " Handle input events for the age input."] # [doc = ""] # [doc = " Digits are accepted as input, with any input which would exceed the maximum age being"] # [doc = " ignored. The up/down arrow keys and 'j'/'k' keys can be used to increment/decrement the"] # [doc = " age."] fn on_key_press (& mut self , event : KeyEvent) { match event . code { KeyCode :: Char (digit @ '0' ..= '9') => { let value = self . value . saturating_mul (10) . saturating_add (digit as u8 - b'0') ; if value <= Self :: MAX { self . value = value ; } } KeyCode :: Backspace => self . value /= 10 , KeyCode :: Up | KeyCode :: Char ('k') => self . increment () , KeyCode :: Down | KeyCode :: Char ('j') => self . decrement () , _ => { } } } fn increment (& mut self) { self . value = self . value . saturating_add (1) . min (Self :: MAX) ; } const fn decrement (& mut self) { self . value = self . value . saturating_sub (1) ; } fn cursor_offset (& self) -> Offset { let x = (self . label . len () + self . value . to_string () . len () + 2) as i32 ; Offset :: new (x , 0) } }
};
}
