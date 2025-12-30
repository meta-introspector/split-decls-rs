// Generated macro for impl_56 (impl)
macro_rules! Depcrate_regex_bytesimpl_56 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_56"}
// Dependencies: {}
# [doc = " Get a matching capture group's haystack substring by name."] # [doc = ""] # [doc = " The haystack substring returned can't outlive the `Captures` object if this"] # [doc = " method is used, because of how `Index` is defined (normally `a[i]` is part"] # [doc = " of `a` and can't outlive it). To work around this limitation, do that, use"] # [doc = " [`Captures::name`] instead."] # [doc = ""] # [doc = " `'h` is the lifetime of the matched haystack, but the lifetime of the"] # [doc = " `&str` returned by this implementation is the lifetime of the `Captures`"] # [doc = " value itself."] # [doc = ""] # [doc = " `'n` is the lifetime of the group name used to index the `Captures` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If there is no matching group at the given name."] impl < 'h , 'n > core :: ops :: Index < & 'n str > for Captures < 'h > { type Output = [u8] ; fn index < 'a > (& 'a self , name : & 'n str) -> & 'a [u8] { self . name (name) . map (| m | m . as_bytes ()) . unwrap_or_else (| | panic ! ("no group named '{name}'")) } }
};
}
