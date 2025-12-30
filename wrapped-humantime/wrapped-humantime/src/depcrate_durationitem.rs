// Generated macro for item (function)
macro_rules! Depcrate_durationitem {
() => {
// Module: crate::duration
// Provides: {"item"}
// Dependencies: {}
fn item (f : & mut fmt :: Formatter , started : & mut bool , name : & str , value : u32) -> fmt :: Result { if value > 0 { if * started { f . write_str (" ") ? ; } write ! (f , "{}{}" , value , name) ? ; * started = true ; } Ok (()) }
};
}
