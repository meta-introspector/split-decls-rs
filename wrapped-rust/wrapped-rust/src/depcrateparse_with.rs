// Generated macro for parse_with (function)
macro_rules! Depcrateparse_with {
() => {
// Module: crate
// Provides: {"parse_with"}
// Dependencies: {}
# [cfg (feature = "clap")] fn parse_with (s : & str) -> Result < (String , WithOption) , String > { let (k , v) = s . split_once ('=') . ok_or_else (| | { format ! ("expected string of form `<key>=<value>[,<key>=<value>...]`; got `{s}`") }) ? ; let v = match v { "generate" => WithOption :: Generate , other => WithOption :: Path (other . to_string ()) , } ; Ok ((k . to_string () , v)) }
};
}
