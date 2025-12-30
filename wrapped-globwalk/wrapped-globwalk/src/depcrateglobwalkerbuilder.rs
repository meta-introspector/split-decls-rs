// Generated macro for GlobWalkerBuilder (struct)
macro_rules! DepcrateGlobWalkerBuilder {
() => {
// Module: crate
// Provides: {"GlobWalkerBuilder"}
// Dependencies: {}
# [doc = " An iterator for recursively yielding glob matches."] # [doc = ""] # [doc = " The order of elements yielded by this iterator is unspecified."] pub struct GlobWalkerBuilder { root : PathBuf , patterns : Vec < String > , walker : WalkDir , case_insensitive : bool , file_type : Option < FileType > , }
};
}
