// Generated macro for capitalize (function)
macro_rules! Depcratecapitalize {
() => {
// Module: crate
// Provides: {"capitalize"}
// Dependencies: {}
fn capitalize (s : & str , f : & mut fmt :: Formatter) -> fmt :: Result { let mut char_indices = s . char_indices () ; if let Some ((_ , c)) = char_indices . next () { write ! (f , "{}" , c . to_uppercase ()) ? ; if let Some ((i , _)) = char_indices . next () { lowercase (& s [i ..] , f) ? ; } } Ok (()) }
};
}
