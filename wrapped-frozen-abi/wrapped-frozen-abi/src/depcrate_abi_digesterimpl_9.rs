// Generated macro for impl_9 (impl)
macro_rules! Depcrate_abi_digesterimpl_9 {
() => {
// Module: crate::abi_digester
// Provides: {"impl_9"}
// Dependencies: {}
impl DigestError { pub (crate) fn wrap_by_type < T : ? Sized > (e : DigestError) -> DigestError { DigestError :: Node (type_name :: < T > () , Box :: new (e)) } pub (crate) fn wrap_by_str (e : DigestError , s : Sstr) -> DigestError { DigestError :: Node (s , Box :: new (e)) } }
};
}
