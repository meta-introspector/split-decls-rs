// Generated macro for PasswordReplaced (struct)
macro_rules! Depcrate_mask_passwordPasswordReplaced {
() => {
// Module: crate::mask_password
// Provides: {"PasswordReplaced"}
// Dependencies: {}
# [doc = " A wrapper of an IRI string that replaces the non-empty password when `Display`ed."] # [doc = ""] # [doc = " This is a retrun type of `mask_password` method of IRI string types (such as"] # [doc = " [`RiStr::mask_password`])."] # [doc = ""] # [doc = " Note that the result might be invalid as an IRI since arbitrary string can"] # [doc = " go to the place of the password."] # [cfg_attr (feature = "alloc" , doc = "Because of this, [`ToDedicatedString`] trait is not implemented for this type.")] # [doc = ""] # [doc = " [`PasswordMasked::replace_password`]: `PasswordMasked::replace_password`"] pub struct PasswordReplaced < 'a , T : ? Sized , D > { # [doc = " IRI reference."] iri_ref : & 'a T , # [doc = " Password range and alternative content."] password : Option < (Range < usize > , D) > , }
};
}
