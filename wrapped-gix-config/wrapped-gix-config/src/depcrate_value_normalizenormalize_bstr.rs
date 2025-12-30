// Generated macro for normalize_bstr (function)
macro_rules! Depcrate_value_normalizenormalize_bstr {
() => {
// Module: crate::value::normalize
// Provides: {"normalize_bstr"}
// Dependencies: {}
# [doc = " `&[u8]` variant of [`normalize`]."] # [must_use] pub fn normalize_bstr < 'a > (input : impl Into < & 'a BStr >) -> Cow < 'a , BStr > { normalize (Cow :: Borrowed (input . into ())) }
};
}
