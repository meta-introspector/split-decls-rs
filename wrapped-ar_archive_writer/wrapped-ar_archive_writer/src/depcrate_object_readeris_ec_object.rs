// Generated macro for is_ec_object (function)
macro_rules! Depcrate_object_readeris_ec_object {
() => {
// Module: crate::object_reader
// Provides: {"is_ec_object"}
// Dependencies: {}
pub fn is_ec_object (obj : & [u8]) -> bool { match object :: FileKind :: parse (obj) { Ok (object :: FileKind :: Coff) => { u16 :: from_le_bytes ([obj [0] , obj [1]]) != object :: pe :: IMAGE_FILE_MACHINE_ARM64 } Ok (object :: FileKind :: CoffImport) => { u16 :: from_le_bytes ([obj [6] , obj [7]]) != object :: pe :: IMAGE_FILE_MACHINE_ARM64 } _ => false , } }
};
}
