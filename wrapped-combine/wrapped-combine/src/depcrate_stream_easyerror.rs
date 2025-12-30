// Generated macro for Error (enum)
macro_rules! Depcrate_stream_easyError {
() => {
// Module: crate::stream::easy
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Enum used to store information about an error that has occurred during parsing."] # [derive (Debug)] pub enum Error < T , R > { # [doc = " Error indicating an unexpected token has been encountered in the stream"] Unexpected (Info < T , R >) , # [doc = " Error indicating that the parser expected something else"] Expected (Info < T , R >) , # [doc = " Generic message"] Message (Info < T , R >) , # [doc = " Variant for containing other types of errors"] Other (Box < dyn StdError + Send + Sync >) , }
};
}
