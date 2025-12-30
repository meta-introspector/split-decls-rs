// Generated macro for ReadTextResult (enum)
macro_rules! Depcrate_readerReadTextResult {
() => {
// Module: crate::reader
// Provides: {"ReadTextResult"}
// Dependencies: {}
# [doc = " Result of an attempt to read XML textual data from the source."] # [derive (Debug)] enum ReadTextResult < 'r , B > { # [doc = " Start of markup (`<` character) was found in the first byte. `<` was consumed."] # [doc = " Contains buffer that should be returned back to the next iteration cycle"] # [doc = " to satisfy borrow checker requirements."] Markup (B) , # [doc = " Start of reference (`&` character) was found in the first byte."] # [doc = " `&` was not consumed."] # [doc = " Contains buffer that should be returned back to the next iteration cycle"] # [doc = " to satisfy borrow checker requirements."] Ref (B) , # [doc = " Contains text block up to start of markup (`<` character). `<` was consumed."] UpToMarkup (& 'r [u8]) , # [doc = " Contains text block up to start of reference (`&` character)."] # [doc = " `&` was not consumed."] UpToRef (& 'r [u8]) , # [doc = " Contains text block up to EOF, neither start of markup (`<` character)"] # [doc = " or start of reference (`&` character) was found."] UpToEof (& 'r [u8]) , # [doc = " IO error occurred."] Err (io :: Error) , }
};
}
