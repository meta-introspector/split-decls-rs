// Generated macro for impl_142 (impl)
macro_rules! Depcrate_engine_general_purposeimpl_142 {
() => {
// Module: crate::engine::general_purpose
// Provides: {"impl_142"}
// Dependencies: {}
impl GeneralPurpose { # [doc = " Create a `GeneralPurpose` engine from an [Alphabet]."] # [doc = ""] # [doc = " While not very expensive to initialize, ideally these should be cached"] # [doc = " if the engine will be used repeatedly."] # [must_use] pub const fn new (alphabet : & Alphabet , config : GeneralPurposeConfig) -> Self { Self { encode_table : encode_table (alphabet) , decode_table : decode_table (alphabet) , config , } } }
};
}
