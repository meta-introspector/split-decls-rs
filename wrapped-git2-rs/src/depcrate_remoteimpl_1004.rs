// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_remoteimpl_1004 {
() => {
// Module: crate::remote
// Provides: {"impl_1004"}
// Dependencies: {}
# [allow (missing_docs)] impl < 'remote > RemoteHead < 'remote > { # [doc = " Flag if this is available locally."] pub fn is_local (& self) -> bool { unsafe { (* self . raw) . local != 0 } } pub fn oid (& self) -> Oid { unsafe { Binding :: from_raw (& (* self . raw) . oid as * const _) } } pub fn loid (& self) -> Oid { unsafe { Binding :: from_raw (& (* self . raw) . loid as * const _) } } pub fn name (& self) -> & str { let b = unsafe { crate :: opt_bytes (self , (* self . raw) . name) . unwrap () } ; str :: from_utf8 (b) . unwrap () } pub fn symref_target (& self) -> Option < & str > { let b = unsafe { crate :: opt_bytes (self , (* self . raw) . symref_target) } ; b . map (| b | str :: from_utf8 (b) . unwrap ()) } }
};
}
