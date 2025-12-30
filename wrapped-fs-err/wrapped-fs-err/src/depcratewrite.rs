// Generated macro for write (function)
macro_rules! Depcratewrite {
() => {
// Module: crate
// Provides: {"write"}
// Dependencies: {}
# [doc = " Write a slice as the entire contents of a file."] # [doc = ""] # [doc = " Wrapper for [`fs::write`](https://doc.rust-lang.org/stable/std/fs/fn.write.html)."] pub fn write < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) -> io :: Result < () > { let path = path . as_ref () ; file :: create (path) . map_err (| err_gen | err_gen (path . to_path_buf ())) ? . write_all (contents . as_ref ()) . map_err (| err | Error :: build (err , ErrorKind :: Write , path)) }
};
}
