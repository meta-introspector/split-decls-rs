// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
enum Error { IncorrectUsage , ReadFile (io :: Error) , ParseFile { error : syn :: Error , filepath : PathBuf , source_code : String , } , }
};
}
