// Generated macro for from_slice_dyn (function)
macro_rules! Depcrate_defrom_slice_dyn {
() => {
// Module: crate::de
// Provides: {"from_slice_dyn"}
// Dependencies: {}
pub fn from_slice_dyn (schema : & OwnedDataModelType , data : & [u8]) -> Result < Value , Error > { let (val , _remain) = deserialize (schema , data) ? ; Ok (val) }
};
}
