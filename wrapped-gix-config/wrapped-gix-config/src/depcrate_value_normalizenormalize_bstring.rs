// Generated macro for normalize_bstring (function)
macro_rules! Depcrate_value_normalizenormalize_bstring {
() => {
// Module: crate::value::normalize
// Provides: {"normalize_bstring"}
// Dependencies: {}
# [doc = " `Vec[u8]` variant of [`normalize`]."] # [must_use] pub fn normalize_bstring (input : impl Into < BString >) -> Cow < 'static , BStr > { normalize (Cow :: Owned (input . into ())) }
};
}
