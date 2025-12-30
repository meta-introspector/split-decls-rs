// Generated macro for get_env (function)
macro_rules! Depcrateget_env {
() => {
// Module: crate
// Provides: {"get_env"}
// Dependencies: {}
# [doc = " Gets the environment variable named `name` if it is set."] # [doc = ""] # [doc = " Returns `None` if the variable is unset, is empty, or contains only whitespace."] # [doc = ""] # [doc = " Trims whitespace from the start and end of the value before returning it."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if the environment variable value is not UTF-8."] pub fn get_env (name : & str) -> Result < Option < String > , String > { let value = match std :: env :: var (name) { Ok (value) => value , Err (std :: env :: VarError :: NotPresent) => return Ok (None) , Err (std :: env :: VarError :: NotUnicode (_)) => { return Err (format ! ("env var '{name}' contains non-utf8 bytes")) } } ; let trimmed = value . trim () ; if trimmed . is_empty () { return Ok (None) ; } Ok (Some (trimmed . to_string ())) }
};
}
