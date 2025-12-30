// Generated macro for EncoderResult (enum)
macro_rules! DepcrateEncoderResult {
() => {
// Module: crate
// Provides: {"EncoderResult"}
// Dependencies: {}
# [doc = " Result of a (potentially partial) encode operation without replacement."] # [must_use] # [derive (Debug , PartialEq , Eq)] pub enum EncoderResult { # [doc = " The input was exhausted."] # [doc = ""] # [doc = " If this result was returned from a call where `last` was `true`, the"] # [doc = " decoding process has completed. Otherwise, the caller should call a"] # [doc = " decode method again with more input."] InputEmpty , # [doc = " The encoder cannot produce another unit of output, because the output"] # [doc = " buffer does not have enough space left."] # [doc = ""] # [doc = " The caller must provide more output space upon the next call and re-push"] # [doc = " the remaining input to the decoder."] OutputFull , # [doc = " The encoder encountered an unmappable character."] # [doc = ""] # [doc = " The caller must either treat this as a fatal error or must append"] # [doc = " a placeholder to the output and then re-push the remaining input to the"] # [doc = " encoder."] Unmappable (char) , }
};
}
