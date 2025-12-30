// Generated macro for ReaderEofState (enum)
macro_rules! Depcrate_readerReaderEofState {
() => {
// Module: crate::reader
// Provides: {"ReaderEofState"}
// Dependencies: {}
# [doc = " Whether EOF of the underlying reader has been reached or not."] # [doc = ""] # [doc = " IO errors on the underlying reader will be considered as an EOF for"] # [doc = " subsequent read attempts, as it would be incorrect to keep on trying"] # [doc = " to read when the underlying reader has broken."] # [doc = ""] # [doc = " For clarity, having the best `Debug` impl and in case they need to be"] # [doc = " treated differently at some point, we store whether the `EOF` is"] # [doc = " considered because an actual EOF happened, or because we encoundered"] # [doc = " an IO error"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum ReaderEofState { NotEof , Eof , IOError , }
};
}
