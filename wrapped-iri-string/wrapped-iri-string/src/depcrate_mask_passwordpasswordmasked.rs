// Generated macro for PasswordMasked (struct)
macro_rules! Depcrate_mask_passwordPasswordMasked {
() => {
// Module: crate::mask_password
// Provides: {"PasswordMasked"}
// Dependencies: {}
# [doc = " A wrapper of an IRI string that masks the non-empty password when `Display`ed."] # [doc = ""] # [doc = " This is a retrun type of `mask_password` method of IRI string types (such as"] # [doc = " [`RiStr::mask_password`])."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use iri_string::validate::Error;"] # [doc = " # #[cfg(feature = \"alloc\")] {"] # [doc = " use iri_string::types::UriReferenceStr;"] # [doc = ""] # [doc = " let iri = UriReferenceStr::new(\"http://user:password@example.com/path?query\")?;"] # [doc = " let masked = iri.mask_password();"] # [doc = " assert_eq!(masked.to_string(), \"http://user:@example.com/path?query\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     masked.replace_password(\"${password}\").to_string(),"] # [doc = "     \"http://user:${password}@example.com/path?query\""] # [doc = " );"] # [doc = " # }"] # [doc = " # Ok::<_, Error>(())"] # [doc = " ```"] # [doc = ""] # [doc = " [`RiStr::mask_password`]: `crate::types::RiStr::mask_password`"] # [derive (Clone , Copy)] pub struct PasswordMasked < 'a , T : ? Sized > { # [doc = " IRI reference."] iri_ref : & 'a T , }
};
}
