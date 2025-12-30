// Generated macro for change_to_char (function)
macro_rules! Depcrate_repository_statuschange_to_char {
() => {
// Module: crate::repository::status
// Provides: {"change_to_char"}
// Dependencies: {}
fn change_to_char (change : & Change < () , gix :: submodule :: Status >) -> u8 { match change { Change :: Removed => b'D' , Change :: Type { .. } => b'T' , Change :: SubmoduleModification (_) => b'M' , Change :: Modification { executable_bit_changed , .. } => { if * executable_bit_changed { b'X' } else { b'M' } } } }
};
}
