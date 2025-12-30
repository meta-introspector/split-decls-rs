// Generated macro for InitializePsProtection (function)
macro_rules! Depcrate_ntpsapiInitializePsProtection {
() => {
// Module: crate::ntpsapi
// Provides: {"InitializePsProtection"}
// Dependencies: {}
# [inline] pub fn InitializePsProtection (aProtectionLevelPtr : & mut PS_PROTECTION , aSigner : PS_PROTECTED_SIGNER , aAudit : u8 , aType : PS_PROTECTED_TYPE ,) { aProtectionLevelPtr . set_Signer (aSigner as u8) ; aProtectionLevelPtr . set_Audit (aAudit) ; aProtectionLevelPtr . set_Type (aType as u8) ; }
};
}
