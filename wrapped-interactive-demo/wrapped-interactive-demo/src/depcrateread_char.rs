// Generated macro for read_char (function)
macro_rules! Depcrateread_char {
() => {
// Module: crate
// Provides: {"read_char"}
// Dependencies: {}
pub fn read_char () -> std :: io :: Result < char > { loop { if let Ok (Event :: Key (KeyEvent { code : KeyCode :: Char (c) , kind : KeyEventKind :: Press , modifiers : _ , state : _ , })) = event :: read () { return Ok (c) ; } } }
};
}
