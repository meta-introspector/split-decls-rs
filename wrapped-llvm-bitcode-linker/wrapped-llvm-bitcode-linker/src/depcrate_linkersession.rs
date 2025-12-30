// Generated macro for Session (struct)
macro_rules! Depcrate_linkerSession {
() => {
// Module: crate::linker
// Provides: {"Session"}
// Dependencies: {}
# [derive (Debug)] pub struct Session { target : Target , cpu : Option < String > , feature : Option < String > , symbols : Vec < String > , # [doc = " A file that `llvm-link` supports, like a bitcode file or an archive."] files : Vec < PathBuf > , link_path : PathBuf , opt_path : PathBuf , sym_path : PathBuf , out_path : PathBuf , }
};
}
