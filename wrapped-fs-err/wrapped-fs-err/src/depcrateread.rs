// Generated macro for read (function)
macro_rules! Depcrateread {
() => {
// Module: crate
// Provides: {"read"}
// Dependencies: {}
# [doc = " Read the entire contents of a file into a bytes vector."] # [doc = ""] # [doc = " Wrapper for [`fs::read`](https://doc.rust-lang.org/stable/std/fs/fn.read.html)."] pub fn read < P : AsRef < Path > > (path : P) -> io :: Result < Vec < u8 > > { let path = path . as_ref () ; let mut file = file :: open (path) . map_err (| err_gen | err_gen (path . to_path_buf ())) ? ; let mut bytes = Vec :: with_capacity (initial_buffer_size (& file)) ; file . read_to_end (& mut bytes) . map_err (| err | Error :: build (err , ErrorKind :: Read , path)) ? ; Ok (bytes) }
};
}
