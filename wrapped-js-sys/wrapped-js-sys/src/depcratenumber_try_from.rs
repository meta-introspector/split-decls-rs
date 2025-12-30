// Generated macro for number_try_from (macro)
macro_rules! Depcratenumber_try_from {
() => {
// Module: crate
// Provides: {"number_try_from"}
// Dependencies: {}
macro_rules ! number_try_from { ($ ($ x : ident) *) => ($ (impl TryFrom <$ x > for Number { type Error = TryFromIntError ; # [inline] fn try_from (x : $ x) -> Result < Number , Self :: Error > { let x_f64 = x as f64 ; if (Number :: MIN_SAFE_INTEGER ..= Number :: MAX_SAFE_INTEGER) . contains (& x_f64) { Ok (Number :: from (x_f64)) } else { Err (TryFromIntError (())) } } }) *) }
};
}
