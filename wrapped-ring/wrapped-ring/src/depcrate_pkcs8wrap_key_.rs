// Generated macro for wrap_key_ (function)
macro_rules! Depcrate_pkcs8wrap_key_ {
() => {
// Module: crate::pkcs8
// Provides: {"wrap_key_"}
// Dependencies: {}
# [doc = " Formats a private key \"prefix||private_key||middle||public_key\" where"] # [doc = " `template` is \"prefix||middle\" split at position `private_key_index`."] fn wrap_key_ (template : & Template , private_key : & [u8] , public_key : & [u8] , bytes : & mut [u8]) { let (before_private_key , after_private_key) = template . bytes . split_at (template . private_key_index) ; let private_key_end_index = template . private_key_index + private_key . len () ; bytes [.. template . private_key_index] . copy_from_slice (before_private_key) ; bytes [template . private_key_index .. private_key_end_index] . copy_from_slice (private_key) ; bytes [private_key_end_index .. (private_key_end_index + after_private_key . len ())] . copy_from_slice (after_private_key) ; bytes [(private_key_end_index + after_private_key . len ()) ..] . copy_from_slice (public_key) ; }
};
}
