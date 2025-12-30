// Generated macro for map_err (function)
macro_rules! Depcrate_codec_framed_readmap_err {
() => {
// Module: crate::codec::framed_read
// Provides: {"map_err"}
// Dependencies: {}
fn map_err (err : io :: Error) -> Error { if let io :: ErrorKind :: InvalidData = err . kind () { if let Some (custom) = err . get_ref () { if custom . is :: < LengthDelimitedCodecError > () { return Error :: library_go_away (Reason :: FRAME_SIZE_ERROR) ; } } } err . into () }
};
}
