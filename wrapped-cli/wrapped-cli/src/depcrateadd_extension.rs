// Generated macro for add_extension (function)
macro_rules! Depcrateadd_extension {
() => {
// Module: crate
// Provides: {"add_extension"}
// Dependencies: {}
# [doc = " A temporary utility function that appends a file extension"] # [doc = " to the provided path buf."] # [doc = ""] # [doc = " Pending removal when our MSRV reaches 1.91 so we can use"] # [doc = ""] # [doc = " <https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.add_extension>"] fn add_extension < P : AsRef < Path > > (path : & Path , extension : P) -> PathBuf { let mut output = path . to_path_buf () . into_os_string () ; output . push (extension . as_ref () . as_os_str ()) ; output . into () }
};
}
