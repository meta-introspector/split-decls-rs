// Generated macro for get_one (function)
macro_rules! Depcrate_directiveget_one {
() => {
// Module: crate::directive
// Provides: {"get_one"}
// Dependencies: {}
fn get_one < 'a > (matches : & [& 'a Value]) -> Result < & 'a Value , String > { match matches { [] => Err ("matched to no values" . to_owned ()) , [matched] => Ok (matched) , _ => Err (format ! ("matched to multiple values {matches:?}, but want exactly 1")) , } }
};
}
