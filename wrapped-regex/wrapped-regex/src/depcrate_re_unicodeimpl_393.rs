// Generated macro for impl_393 (impl)
macro_rules! Depcrate_re_unicodeimpl_393 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_393"}
// Dependencies: {}
# [doc = " Get a group by index."] # [doc = ""] # [doc = " `'t` is the lifetime of the matched text."] # [doc = ""] # [doc = " The text can't outlive the `Captures` object if this method is"] # [doc = " used, because of how `Index` is defined (normally `a[i]` is part"] # [doc = " of `a` and can't outlive it); to do that, use `at()` instead."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If there is no group at the given index."] impl < 't > Index < usize > for Captures < 't > { type Output = str ; fn index (& self , i : usize) -> & str { self . at (i) . unwrap_or_else (| | panic ! ("no group at index '{}'" , i)) } }
};
}
