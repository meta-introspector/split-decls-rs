// Generated macro for parse_type_signature (function)
macro_rules! Depcrate_read_unitparse_type_signature {
() => {
// Module: crate::read::unit
// Provides: {"parse_type_signature"}
// Dependencies: {}
# [doc = " Parse a type unit header's unique type signature. Callers should handle"] # [doc = " unique-ness checking."] fn parse_type_signature < R : Reader > (input : & mut R) -> Result < DebugTypeSignature > { input . read_u64 () . map (DebugTypeSignature) }
};
}
