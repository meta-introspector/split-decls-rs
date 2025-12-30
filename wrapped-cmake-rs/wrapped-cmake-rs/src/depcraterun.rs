// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (cmd : & mut Command , program : & str) { eprintln ! ("running: {:?}" , cmd) ; let status = match cmd . status () { Ok (status) => status , Err (ref e) if e . kind () == ErrorKind :: NotFound => { fail (& format ! ("failed to execute command: {}\nis `{}` not installed?" , e , program)) ; } Err (e) => fail (& format ! ("failed to execute command: {}" , e)) , } ; if ! status . success () { if status . code () == Some (127) { fail (& format ! ("command did not execute successfully, got: {}, is `{}` not installed?" , status , program)) ; } fail (& format ! ("command did not execute successfully, got: {}" , status)) ; } }
};
}
