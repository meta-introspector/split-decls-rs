// Generated macro for impl_1449 (impl)
macro_rules! Depcrate_stringimpl_1449 {
() => {
// Module: crate::string
// Provides: {"impl_1449"}
// Dependencies: {}
# [doc = " A convenience impl that delegates to the impl for `&str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(String::from(\"Hello world\").find(\"world\"), Some(6));"] # [doc = " ```"] # [unstable (feature = "pattern" , reason = "API not fully fleshed out and ready to be stabilized" , issue = "27721")] impl < 'b > Pattern for & 'b String { type Searcher < 'a > = < & 'b str as Pattern > :: Searcher < 'a > ; fn into_searcher (self , haystack : & str) -> < & 'b str as Pattern > :: Searcher < '_ > { self [..] . into_searcher (haystack) } # [inline] fn is_contained_in (self , haystack : & str) -> bool { self [..] . is_contained_in (haystack) } # [inline] fn is_prefix_of (self , haystack : & str) -> bool { self [..] . is_prefix_of (haystack) } # [inline] fn strip_prefix_of (self , haystack : & str) -> Option < & str > { self [..] . strip_prefix_of (haystack) } # [inline] fn is_suffix_of < 'a > (self , haystack : & 'a str) -> bool where Self :: Searcher < 'a > : core :: str :: pattern :: ReverseSearcher < 'a > , { self [..] . is_suffix_of (haystack) } # [inline] fn strip_suffix_of < 'a > (self , haystack : & 'a str) -> Option < & 'a str > where Self :: Searcher < 'a > : core :: str :: pattern :: ReverseSearcher < 'a > , { self [..] . strip_suffix_of (haystack) } # [inline] fn as_utf8_pattern (& self) -> Option < Utf8Pattern < '_ > > { Some (Utf8Pattern :: StringPattern (self . as_bytes ())) } }
};
}
