// Generated macro for DisplayFragments (struct)
macro_rules! Depcrate_frameDisplayFragments {
() => {
// Module: crate::frame
// Provides: {"DisplayFragments"}
// Dependencies: {}
# [doc = " An iterator over the fragments of a log message, formatted as strings."] # [doc = ""] # [doc = " See [`Frame::display_fragments`]."] pub struct DisplayFragments < 't > { frame : & 't Frame < 't > , iter : std :: vec :: IntoIter < Fragment < 't > > , }
};
}
