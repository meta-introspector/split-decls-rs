// Generated macro for GlobResult (type)
macro_rules! DepcrateGlobResult {
() => {
// Module: crate
// Provides: {"GlobResult"}
// Dependencies: {}
# [doc = " An alias for a glob iteration result."] # [doc = ""] # [doc = " This represents either a matched path or a glob iteration error,"] # [doc = " such as failing to read a particular directory's contents."] pub type GlobResult = Result < PathBuf , GlobError > ;
};
}
