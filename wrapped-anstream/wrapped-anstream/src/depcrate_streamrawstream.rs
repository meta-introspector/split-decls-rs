// Generated macro for RawStream (trait)
macro_rules! Depcrate_streamRawStream {
() => {
// Module: crate::stream
// Provides: {"RawStream"}
// Dependencies: {}
# [doc = " Required functionality for underlying [`std::io::Write`] for adaptation"] # [cfg (all (windows , feature = "wincon"))] pub trait RawStream : std :: io :: Write + IsTerminal + anstyle_wincon :: WinconStream + private :: Sealed { }
};
}
