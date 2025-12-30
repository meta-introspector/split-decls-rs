// Generated macro for impl_1572 (impl)
macro_rules! Depcrate_x509impl_1572 {
() => {
// Module: crate::x509
// Provides: {"impl_1572"}
// Dependencies: {}
impl X509NameEntryRef { # [doc = " Returns the field value of an `X509NameEntry`."] # [corresponds (X509_NAME_ENTRY_get_data)] pub fn data (& self) -> & Asn1StringRef { unsafe { let data = ffi :: X509_NAME_ENTRY_get_data (self . as_ptr ()) ; Asn1StringRef :: from_ptr (data) } } # [doc = " Returns the `Asn1Object` value of an `X509NameEntry`."] # [doc = " This is useful for finding out about the actual `Nid` when iterating over all `X509NameEntries`."] # [corresponds (X509_NAME_ENTRY_get_object)] pub fn object (& self) -> & Asn1ObjectRef { unsafe { let object = ffi :: X509_NAME_ENTRY_get_object (self . as_ptr ()) ; Asn1ObjectRef :: from_ptr (object) } } }
};
}
