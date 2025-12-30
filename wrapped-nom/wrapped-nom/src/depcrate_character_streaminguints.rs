// Generated macro for uints (macro)
macro_rules! Depcrate_character_streaminguints {
() => {
// Module: crate::character::streaming
// Provides: {"uints"}
// Dependencies: {}
# [doc (hidden)] macro_rules ! uints { ($ ($ t : tt) +) => { $ (# [doc = " will parse a number in text form to a number"] # [doc = ""] # [doc = " *Complete version*: can parse until the end of input."] pub fn $ t < T , E : ParseError < T >> (input : T) -> IResult < T , $ t , E > where T : Input , < T as Input >:: Item : AsChar , { let i = input ; if i . input_len () == 0 { return Err (Err :: Incomplete (Needed :: new (1))) ; } let mut value : $ t = 0 ; let mut pos = 0 ; for c in i . iter_elements () { match c . as_char () . to_digit (10) { None => { if pos == 0 { return Err (Err :: Error (E :: from_error_kind (i , ErrorKind :: Digit))) ; } else { return Ok ((i . take_from (pos) , value)) ; } } , Some (d) => match value . checked_mul (10) . and_then (| v | v . checked_add (d as $ t)) { None => return Err (Err :: Error (E :: from_error_kind (i , ErrorKind :: Digit))) , Some (v) => { pos += c . len () ; value = v ; } , } } } Err (Err :: Incomplete (Needed :: new (1))) }) + } }
};
}
