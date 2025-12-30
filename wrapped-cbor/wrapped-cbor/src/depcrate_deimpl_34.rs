// Generated macro for impl_34 (impl)
macro_rules! Depcrate_deimpl_34 {
() => {
// Module: crate::de
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'de , 'a , R > MakeError for SeqAccess < 'a , R > where R : Read < 'de > , { fn error (& self , code : ErrorCode) -> Error { self . de . error (code) } }
};
}
