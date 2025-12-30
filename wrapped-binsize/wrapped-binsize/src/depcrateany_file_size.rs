// Generated macro for any_file_size (function)
macro_rules! Depcrateany_file_size {
() => {
// Module: crate
// Provides: {"any_file_size"}
// Dependencies: {}
fn any_file_size (filepath : & str) -> Result < bool , std :: io :: Error > { let exists : bool = std :: path :: Path :: new (filepath) . exists () ; if exists { let fsize = fs :: metadata (filepath) ? . len () ; println ! ("{{\"biggerIsBetter\":false,\"name\":{filepath:?},\"unit\":\"bytes\",\"value\":{fsize}}}") ; } Ok (exists) }
};
}
