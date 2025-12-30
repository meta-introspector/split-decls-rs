// Generated macro for StreamInner (enum)
macro_rules! Depcrate_autoStreamInner {
() => {
// Module: crate::auto
// Provides: {"StreamInner"}
// Dependencies: {}
# [derive (Debug)] enum StreamInner < S : RawStream > { PassThrough (S) , Strip (StripStream < S >) , # [cfg (all (windows , feature = "wincon"))] Wincon (WinconStream < S >) , }
};
}
