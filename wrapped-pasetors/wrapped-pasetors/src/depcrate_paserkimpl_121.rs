// Generated macro for impl_121 (impl)
macro_rules! Depcrate_paserkimpl_121 {
() => {
// Module: crate::paserk
// Provides: {"impl_121"}
// Dependencies: {}
# [cfg (any (feature = "v2" , feature = "v3" , feature = "v4"))] impl TryFrom < & str > for Id { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let split = value . split ('.') . collect :: < Vec < & str > > () ; if split . len () != 3 { return Err (Error :: PaserkParsing) ; } let header = match (split [0] , split [1]) { ("k2" , "lid" | "sid" | "pid") | ("k3" , "sid" | "pid") | ("k4" , "lid" | "sid" | "pid") => format ! ("{}.{}." , split [0] , split [1]) , _ => return Err (Error :: PaserkParsing) , } ; let expected_len = match split [0] { # [cfg (feature = "v2")] "k2" => V2 :: PASERK_ID , # [cfg (feature = "v3")] "k3" => V3 :: PASERK_ID , # [cfg (feature = "v4")] "k4" => V4 :: PASERK_ID , _ => return Err (Error :: PaserkParsing) , } ; if split [2] . len () != expected_len { return Err (Error :: PaserkParsing) ; } Ok (Self { header , identifier : split [2] . to_string () , }) } }
};
}
