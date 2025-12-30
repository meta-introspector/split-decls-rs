// Generated macro for lines (function)
macro_rules! Depcratelines {
() => {
// Module: crate
// Provides: {"lines"}
// Dependencies: {}
# [doc = " Computes the diff between the lines of two strings."] pub fn lines < 'a > (left : & 'a str , right : & 'a str) -> Vec < Result < & 'a str > > { let mut diff = do_diff (& left . lines () . collect :: < Vec < _ > > () , & right . lines () . collect :: < Vec < _ > > () , | str | * str ,) ; match (left . as_bytes () . last () . cloned () , right . as_bytes () . last () . cloned () ,) { (Some (b'\n') , Some (b'\n')) => { diff . push (Result :: Both (& left [left . len () ..] , & right [right . len () ..])) } (Some (b'\n') , _) => diff . push (Result :: Left (& left [left . len () ..])) , (_ , Some (b'\n')) => diff . push (Result :: Right (& right [right . len () ..])) , _ => { } } diff }
};
}
