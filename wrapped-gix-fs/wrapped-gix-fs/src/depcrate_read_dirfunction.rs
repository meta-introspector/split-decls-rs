// Generated macro for function (module)
macro_rules! Depcrate_read_dirfunction {
() => {
// Module: crate::read_dir
// Provides: {"function"}
// Dependencies: {}
pub (crate) mod function { use std :: path :: Path ; # [doc = " List all entries in `path`, similar to [`std::fs::read_dir()`], and assure all available information"] # [doc = " adheres to the value of `precompose_unicode`."] pub fn read_dir (path : & Path , precompose_unicode : bool ,) -> std :: io :: Result < impl Iterator < Item = std :: io :: Result < super :: DirEntry > > > { std :: fs :: read_dir (path) . map (move | it | it . map (move | res | res . map (| entry | super :: DirEntry :: new (entry , precompose_unicode)))) } }
};
}
