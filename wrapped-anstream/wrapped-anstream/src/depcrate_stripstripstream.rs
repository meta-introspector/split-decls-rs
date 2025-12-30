// Generated macro for StripStream (struct)
macro_rules! Depcrate_stripStripStream {
() => {
// Module: crate::strip
// Provides: {"StripStream"}
// Dependencies: {}
# [doc = " Only pass printable data to the inner `Write`"] # [derive (Debug)] pub struct StripStream < S > where S : std :: io :: Write , { raw : S , state : StripBytes , }
};
}
