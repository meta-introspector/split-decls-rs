// Generated macro for ccc (macro)
macro_rules! Depcrateccc {
() => {
// Module: crate
// Provides: {"ccc"}
// Dependencies: {}
macro_rules ! ccc { ($ name : ident , $ num : expr) => { const { # [cfg (feature = "icu_properties")] if icu_properties :: props :: CanonicalCombiningClass ::$ name . to_icu4c_value () != $ num { panic ! ("icu_normalizer has incorrect ccc values") } CanonicalCombiningClass :: from_icu4c_value ($ num) } } ; }
};
}
