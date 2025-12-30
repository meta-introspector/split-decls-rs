// Generated macro for read_to_string (function)
macro_rules! Depcrateread_to_string {
() => {
// Module: crate
// Provides: {"read_to_string"}
// Dependencies: {}
# [doc = " Read the entire contents of a file into a string."] # [doc = ""] # [doc = " Wrapper for [`fs::read_to_string`](https://doc.rust-lang.org/stable/std/fs/fn.read_to_string.html)."] pub fn read_to_string < P : AsRef < Path > > (path : P) -> io :: Result < String > { let path = path . as_ref () ; let mut file = file :: open (path) . map_err (| err_gen | err_gen (path . to_path_buf ())) ? ; let mut string = String :: with_capacity (initial_buffer_size (& file)) ; file . read_to_string (& mut string) . map_err (| err | Error :: build (err , ErrorKind :: Read , path)) ? ; Ok (string) }
};
}
