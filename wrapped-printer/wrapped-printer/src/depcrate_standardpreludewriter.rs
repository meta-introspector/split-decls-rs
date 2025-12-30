// Generated macro for PreludeWriter (struct)
macro_rules! Depcrate_standardPreludeWriter {
() => {
// Module: crate::standard
// Provides: {"PreludeWriter"}
// Dependencies: {}
# [doc = " A writer for the prelude (the beginning part of a matching line)."] # [doc = ""] # [doc = " This encapsulates the state needed to print the prelude."] struct PreludeWriter < 'a , M : Matcher , W > { std : & 'a StandardImpl < 'a , M , W > , next_separator : PreludeSeparator , field_separator : & 'a [u8] , interp_status : hyperlink :: InterpolatorStatus , }
};
}
