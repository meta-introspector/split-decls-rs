// Generated macro for EXTENDABLE_OUTPUT_FUNCTION (static)
macro_rules! Depcrate_typesEXTENDABLE_OUTPUT_FUNCTION {
() => {
// Module: crate::types
// Provides: {"EXTENDABLE_OUTPUT_FUNCTION"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL)))] pub static EXTENDABLE_OUTPUT_FUNCTION : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.primitives.hashes" , & ["ExtendableOutputFunction"] ,) ;
};
}
