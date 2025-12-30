// Generated macro for create_file_recursive (function)
macro_rules! Depcratecreate_file_recursive {
() => {
// Module: crate
// Provides: {"create_file_recursive"}
// Dependencies: {}
fn create_file_recursive (filename : & str) -> std :: io :: Result < File > { let path = std :: path :: Path :: new (filename) ; if let Some (dir) = path . parent () { std :: fs :: create_dir_all (dir) ? ; } File :: create (filename) }
};
}
