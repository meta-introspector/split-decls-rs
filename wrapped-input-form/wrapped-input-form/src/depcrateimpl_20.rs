// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl StringField { const fn new (label : & 'static str) -> Self { Self { label , value : String :: new () , } } # [doc = " Handle input events for the string input."] fn on_key_press (& mut self , event : KeyEvent) { match event . code { KeyCode :: Char (c) => self . value . push (c) , KeyCode :: Backspace => { self . value . pop () ; } _ => { } } } fn cursor_offset (& self) -> Offset { let x = (self . label . len () + self . value . len () + 2) as i32 ; Offset :: new (x , 0) } }
};
}
