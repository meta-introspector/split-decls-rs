// Generated macro for pi (function)
macro_rules! Depcrate_parser_pipi {
() => {
// Module: crate::parser::pi
// Provides: {"pi"}
// Dependencies: {}
# [test] fn pi () { use pretty_assertions :: assert_eq ; # [doc = " Returns `Ok(pos)` with the position in the buffer where processing"] # [doc = " instruction is ended."] # [doc = ""] # [doc = " Returns `Err(internal_state)` if parsing is not done yet."] fn parse_pi (bytes : & [u8] , had_question_mark : bool) -> Result < usize , bool > { let mut parser = PiParser (had_question_mark) ; match parser . feed (bytes) { Some (i) => Ok (i) , None => Err (parser . 0) , } } assert_eq ! (parse_pi (b"" , false) , Err (false)) ; assert_eq ! (parse_pi (b"" , true) , Err (false)) ; assert_eq ! (parse_pi (b"?" , false) , Err (true)) ; assert_eq ! (parse_pi (b"?" , true) , Err (true)) ; assert_eq ! (parse_pi (b">" , false) , Err (false)) ; assert_eq ! (parse_pi (b">" , true) , Ok (0)) ; assert_eq ! (parse_pi (b"?>" , false) , Ok (1)) ; assert_eq ! (parse_pi (b"?>" , true) , Ok (1)) ; assert_eq ! (parse_pi (b">?>" , false) , Ok (2)) ; assert_eq ! (parse_pi (b">?>" , true) , Ok (0)) ; }
};
}
