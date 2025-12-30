// Generated macro for split_at (function)
macro_rules! Depcrate_extsplit_at {
() => {
// Module: crate::ext
// Provides: {"split_at"}
// Dependencies: {}
# [doc = " Split an `OsStr`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `index` must be at a valid UTF-8 boundary"] pub (crate) unsafe fn split_at (os : & OsStr , index : usize) -> (& OsStr , & OsStr) { unsafe { let bytes = os . as_encoded_bytes () ; let (first , second) = bytes . split_at (index) ; (OsStr :: from_encoded_bytes_unchecked (first) , OsStr :: from_encoded_bytes_unchecked (second) ,) } }
};
}
