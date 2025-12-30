// Generated macro for iter2cstrs (function)
macro_rules! Depcrate_utiliter2cstrs {
() => {
// Module: crate::util
// Provides: {"iter2cstrs"}
// Dependencies: {}
# [doc = " Converts an iterator of things into a git array of c-strings."] # [doc = ""] # [doc = " Returns a tuple `(cstrings, pointers, git_strarray)`. The first two values"] # [doc = " should not be dropped before `git_strarray`."] pub fn iter2cstrs < T , I > (iter : I ,) -> Result < (Vec < CString > , Vec < * const c_char > , raw :: git_strarray) , Error > where T : IntoCString , I : IntoIterator < Item = T > , { let cstrs = iter . into_iter () . map (| i | i . into_c_string ()) . collect :: < Result < Vec < CString > , _ > > () ? ; let ptrs = cstrs . iter () . map (| i | i . as_ptr ()) . collect :: < Vec < _ > > () ; let raw = raw :: git_strarray { strings : ptrs . as_ptr () as * mut _ , count : ptrs . len () as size_t , } ; Ok ((cstrs , ptrs , raw)) }
};
}
