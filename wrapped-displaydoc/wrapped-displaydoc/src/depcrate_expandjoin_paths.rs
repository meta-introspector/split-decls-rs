// Generated macro for join_paths (function)
macro_rules! Depcrate_expandjoin_paths {
() => {
// Module: crate::expand
// Provides: {"join_paths"}
// Dependencies: {}
# [doc = " Create a path with segments composed of [Idents] *without* any [PathArguments]."] fn join_paths (name_segments : & [& str] , use_global_prefix : UseGlobalPrefix) -> Path { let mut segments = Punctuated :: < PathSegment , PathSep > :: new () ; assert ! (! name_segments . is_empty ()) ; segments . push_value (PathSegment { ident : Ident :: new (name_segments [0] , Span :: call_site ()) , arguments : PathArguments :: None , }) ; for name in name_segments [1 ..] . iter () { segments . push_punct (PathSep { spans : [Span :: call_site () , Span :: mixed_site ()] , }) ; segments . push_value (PathSegment { ident : Ident :: new (name , Span :: call_site ()) , arguments : PathArguments :: None , }) ; } Path { leading_colon : match use_global_prefix { UseGlobalPrefix :: LeadingColon => Some (PathSep { spans : [Span :: call_site () , Span :: mixed_site ()] , }) , UseGlobalPrefix :: NoLeadingColon => None , } , segments , } }
};
}
