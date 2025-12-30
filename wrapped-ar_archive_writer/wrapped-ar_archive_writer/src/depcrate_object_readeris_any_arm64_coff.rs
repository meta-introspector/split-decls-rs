// Generated macro for is_any_arm64_coff (function)
macro_rules! Depcrate_object_readeris_any_arm64_coff {
() => {
// Module: crate::object_reader
// Provides: {"is_any_arm64_coff"}
// Dependencies: {}
pub fn is_any_arm64_coff (obj : & [u8]) -> bool { match object :: FileKind :: parse (obj) { Ok (object :: FileKind :: Coff) => u16 :: from_le_bytes ([obj [0] , obj [1]]) . try_into () . is_ok_and (is_any_arm64) , Ok (object :: FileKind :: CoffImport) => { u16 :: from_le_bytes ([obj [6] , obj [7]]) . try_into () . is_ok_and (is_any_arm64) } _ => false , } }
};
}
