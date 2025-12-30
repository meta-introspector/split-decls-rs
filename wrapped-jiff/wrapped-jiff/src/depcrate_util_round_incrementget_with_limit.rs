// Generated macro for get_with_limit (function)
macro_rules! Depcrate_util_round_incrementget_with_limit {
() => {
// Module: crate::util::round::increment
// Provides: {"get_with_limit"}
// Dependencies: {}
fn get_with_limit (unit : Unit , increment : i64 , what : & 'static str , limit : & [t :: Constant] ,) -> Result < t :: NoUnits128 , Error > { let increment = t :: NoUnits :: new_unchecked (increment) ; if increment <= C (0) { return Err (err ! ("rounding increment {increment} for {unit} must be \
             greater than zero" , unit = unit . plural () ,)) ; } let Some (must_divide) = limit . get (unit as usize) else { return Err (err ! ("{what} rounding does not support {unit}" , unit = unit . plural ())) ; } ; let must_divide = t :: NoUnits :: rfrom (* must_divide) ; if increment >= must_divide || must_divide % increment != C (0) { Err (err ! ("increment {increment} for rounding {what} to {unit} \
             must be 1) less than {must_divide}, 2) divide into \
             it evenly and 3) greater than zero" , unit = unit . plural () ,)) } else { Ok (t :: NoUnits128 :: rfrom (increment)) } }
};
}
