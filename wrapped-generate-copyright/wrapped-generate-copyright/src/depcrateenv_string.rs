// Generated macro for env_string (function)
macro_rules! Depcrateenv_string {
() => {
// Module: crate
// Provides: {"env_string"}
// Dependencies: {}
# [doc = " Grab an environment variable as string, or fail nicely."] fn env_string (var : & str) -> Result < String , Error > { match std :: env :: var (var) { Ok (var) => Ok (var) , Err (std :: env :: VarError :: NotUnicode (_)) => { anyhow :: bail ! ("environment variable {var} is not utf-8") } Err (std :: env :: VarError :: NotPresent) => anyhow :: bail ! ("missing environment variable {var}") , } }
};
}
