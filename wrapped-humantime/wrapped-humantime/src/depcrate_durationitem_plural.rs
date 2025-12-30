// Generated macro for item_plural (function)
macro_rules! Depcrate_durationitem_plural {
() => {
// Module: crate::duration
// Provides: {"item_plural"}
// Dependencies: {}
fn item_plural (f : & mut fmt :: Formatter , started : & mut bool , name : & str , value : u64) -> fmt :: Result { if value > 0 { if * started { f . write_str (" ") ? ; } write ! (f , "{}{}" , value , name) ? ; if value > 1 { f . write_str ("s") ? ; } * started = true ; } Ok (()) }
};
}
