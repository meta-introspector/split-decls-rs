// Generated macro for impl_394 (impl)
macro_rules! Depcrate_re_unicodeimpl_394 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_394"}
// Dependencies: {}
# [doc = " Get a group by name."] # [doc = ""] # [doc = " `'t` is the lifetime of the matched text and `'i` is the lifetime"] # [doc = " of the group name (the index)."] # [doc = ""] # [doc = " The text can't outlive the `Captures` object if this method is"] # [doc = " used, because of how `Index` is defined (normally `a[i]` is part"] # [doc = " of `a` and can't outlive it); to do that, use `name` instead."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If there is no group named by the given value."] impl < 't , 'i > Index < & 'i str > for Captures < 't > { type Output = str ; fn index < 'a > (& 'a self , name : & 'i str) -> & 'a str { self . name (name) . unwrap_or_else (| | panic ! ("no group named '{}'" , name)) } }
};
}
