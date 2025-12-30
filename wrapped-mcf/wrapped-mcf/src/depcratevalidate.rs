// Generated macro for validate (function)
macro_rules! Depcratevalidate {
() => {
// Module: crate
// Provides: {"validate"}
// Dependencies: {}
# [doc = " Perform validations that the given string is well-formed MCF."] fn validate (s : & str) -> Result < () > { if ! s . starts_with (fields :: DELIMITER) { return Err (Error { }) ; } if s . ends_with (fields :: DELIMITER) { return Err (Error { }) ; } let mut fields = Fields :: new (s) ; let id = fields . next () . ok_or (Error { }) ? ; validate_id (id . as_str ()) ? ; for field in fields { field . validate () ? ; } Ok (()) }
};
}
