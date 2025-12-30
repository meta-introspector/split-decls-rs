// Generated macro for PsProtectedValue (function)
macro_rules! Depcrate_ntpsapiPsProtectedValue {
() => {
// Module: crate::ntpsapi
// Provides: {"PsProtectedValue"}
// Dependencies: {}
# [inline] pub const fn PsProtectedValue (aSigner : PS_PROTECTED_SIGNER , aAudit : u8 , aType : PS_PROTECTED_TYPE ,) -> UCHAR { (aSigner as u8 & PS_PROTECTED_SIGNER_MASK) << 4 | (aAudit & PS_PROTECTED_AUDIT_MASK) << 3 | (aType as u8 & PS_PROTECTED_TYPE_MASK) }
};
}
