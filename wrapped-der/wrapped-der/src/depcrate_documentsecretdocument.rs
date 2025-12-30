// Generated macro for SecretDocument (struct)
macro_rules! Depcrate_documentSecretDocument {
() => {
// Module: crate::document
// Provides: {"SecretDocument"}
// Dependencies: {}
# [doc = " Secret [`Document`] type."] # [doc = ""] # [doc = " Useful for formats which represent potentially secret data, such as"] # [doc = " cryptographic keys."] # [doc = ""] # [doc = " This type provides additional hardening such as ensuring that the contents"] # [doc = " are zeroized-on-drop, and also using more restrictive file permissions when"] # [doc = " writing files to disk."] # [cfg (feature = "zeroize")] # [derive (Clone)] pub struct SecretDocument (Document) ;
};
}
