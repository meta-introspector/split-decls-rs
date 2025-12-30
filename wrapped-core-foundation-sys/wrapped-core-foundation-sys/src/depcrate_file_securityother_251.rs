// Generated macro for other_251 (other)
macro_rules! Depcrate_file_securityother_251 {
() => {
// Module: crate::file_security
// Provides: {"other_251"}
// Dependencies: {}
unsafe extern "C" { pub fn CFFileSecurityGetTypeID () -> CFTypeID ; pub fn CFFileSecurityCreate (allocator : CFAllocatorRef) -> CFFileSecurityRef ; pub fn CFFileSecurityCreateCopy (allocator : CFAllocatorRef , fileSec : CFFileSecurityRef ,) -> CFFileSecurityRef ; pub fn CFFileSecurityCopyOwnerUUID (fileSec : CFFileSecurityRef , ownerUUID : * mut CFUUIDRef ,) -> Boolean ; pub fn CFFileSecuritySetOwnerUUID (fileSec : CFFileSecurityRef , ownerUUID : CFUUIDRef) -> Boolean ; pub fn CFFileSecurityCopyGroupUUID (fileSec : CFFileSecurityRef , groupUUID : * mut CFUUIDRef ,) -> Boolean ; pub fn CFFileSecuritySetGroupUUID (fileSec : CFFileSecurityRef , groupUUID : CFUUIDRef) -> Boolean ; # [cfg (feature = "mac_os_10_8_features")] # [cfg_attr (feature = "mac_os_10_7_support" , linkage = "extern_weak")] pub fn CFFileSecurityClearProperties (fileSec : CFFileSecurityRef , clearPropertyMask : CFFileSecurityClearOptions ,) -> Boolean ; }
};
}
