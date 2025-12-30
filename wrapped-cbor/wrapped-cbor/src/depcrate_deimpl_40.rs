// Generated macro for impl_40 (impl)
macro_rules! Depcrate_deimpl_40 {
() => {
// Module: crate::de
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'de , 'a , R > MakeError for MapAccess < 'a , R > where R : Read < 'de > , { fn error (& self , code : ErrorCode) -> Error { self . de . error (code) } }
};
}
