// Generated macro for Result (type)
macro_rules! Depcrate_errorResult {
() => {
// Module: crate::error
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized Result type for fs_extra operations."] # [doc = ""] # [doc = " This typedef is generally used to avoid writing out fs_extra::Error directly"] # [doc = " and is otherwise a direct mapping to Result."] # [doc = ""] # [doc = "#Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::create;"] # [doc = ""] # [doc = "fn get_string() -> io::Result<()> {"] # [doc = ""] # [doc = "     create(\"test_dir\")?;"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub type Result < T > = :: std :: result :: Result < T , Error > ;
};
}
