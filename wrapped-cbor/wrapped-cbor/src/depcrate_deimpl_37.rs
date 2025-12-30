// Generated macro for impl_37 (impl)
macro_rules! Depcrate_deimpl_37 {
() => {
// Module: crate::de
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'de , 'a , R > MakeError for IndefiniteSeqAccess < 'a , R > where R : Read < 'de > , { fn error (& self , code : ErrorCode) -> Error { self . de . error (code) } }
};
}
