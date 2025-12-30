// Generated macro for PAD (const)
macro_rules! Depcrate_engine_general_purposePAD {
() => {
// Module: crate::engine::general_purpose
// Provides: {"PAD"}
// Dependencies: {}
# [doc = " Include padding bytes when encoding, and require that they be present when decoding."] # [doc = ""] # [doc = " This is the standard per the base64 RFC, but consider using [`NO_PAD`] or [`NO_PAD_INDIFFERENT`]"] # [doc = " instead as padding serves little purpose in practice."] pub const PAD : GeneralPurposeConfig = GeneralPurposeConfig :: new () ;
};
}
