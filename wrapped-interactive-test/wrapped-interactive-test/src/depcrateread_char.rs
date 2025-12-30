// Generated macro for read_char (function)
macro_rules! Depcrateread_char {
() => {
// Module: crate
// Provides: {"read_char"}
// Dependencies: {}
pub fn read_char () -> Result < char > { loop { if let Event :: Key (KeyEvent { code : KeyCode :: Char (c) , .. }) = event :: read () ? { return Ok (c) ; } } }
};
}
