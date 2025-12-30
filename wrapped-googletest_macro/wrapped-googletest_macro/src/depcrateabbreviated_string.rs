// Generated macro for abbreviated_string (function)
macro_rules! Depcrateabbreviated_string {
() => {
// Module: crate
// Provides: {"abbreviated_string"}
// Dependencies: {}
fn abbreviated_string (target : & str) -> Result < std :: borrow :: Cow < '_ , str > , & 'static str > { use std :: borrow :: Cow ; match target . rsplit_once (',') { None => Err ("Expect a `max_length` argument, but got none") , Some ((expr , limit)) => match limit . trim () . parse :: < usize > () { Ok (limit) if expr . len () > limit => { if limit >= 4 { Ok (Cow :: Owned (format ! ("{}..." , & expr [.. limit - 3]))) } else { Err ("The `max_length` argument is too small. It must be at least 4.") } } Ok (_) => Ok (Cow :: Borrowed (expr)) , Err (_) => Err ("The `max_length` argument is not a number.") , } , } }
};
}
