// Generated macro for write_marker (function)
macro_rules! Depcrate_encodewrite_marker {
() => {
// Module: crate::encode
// Provides: {"write_marker"}
// Dependencies: {}
# [doc = " Attempts to write the given marker into the writer."] fn write_marker < W : RmpWrite > (wr : & mut W , marker : Marker) -> Result < () , MarkerWriteError < W :: Error > > { wr . write_u8 (marker . to_u8 ()) . map_err (MarkerWriteError) }
};
}
