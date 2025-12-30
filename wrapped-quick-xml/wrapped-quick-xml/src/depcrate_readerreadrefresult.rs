// Generated macro for ReadRefResult (enum)
macro_rules! Depcrate_readerReadRefResult {
() => {
// Module: crate::reader
// Provides: {"ReadRefResult"}
// Dependencies: {}
# [doc = " Result of an attempt to read general reference from the reader."] # [derive (Debug)] enum ReadRefResult < 'r > { # [doc = " Contains text block up to end of reference (`;` character)."] # [doc = " Result includes start `&`, but not end `;`."] Ref (& 'r [u8]) , # [doc = " Contains text block up to EOF. Neither end of reference (`;`), start of"] # [doc = " another reference (`&`) or start of markup (`<`) characters was found."] # [doc = " Result includes start `&`."] UpToEof (& 'r [u8]) , # [doc = " Contains text block up to next possible reference (`&` character)."] # [doc = " Result includes start `&`."] UpToRef (& 'r [u8]) , # [doc = " Contains text block up to start of markup (`<` character)."] # [doc = " Result includes start `&`."] UpToMarkup (& 'r [u8]) , # [doc = " IO error occurred."] Err (io :: Error) , }
};
}
