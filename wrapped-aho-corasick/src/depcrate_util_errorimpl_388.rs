// Generated macro for impl_388 (impl)
macro_rules! Depcrate_util_errorimpl_388 {
() => {
// Module: crate::util::error
// Provides: {"impl_388"}
// Dependencies: {}
impl core :: fmt :: Display for MatchError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match * self . kind () { MatchErrorKind :: InvalidInputAnchored => { write ! (f , "anchored searches are not supported or enabled") } MatchErrorKind :: InvalidInputUnanchored => { write ! (f , "unanchored searches are not supported or enabled") } MatchErrorKind :: UnsupportedStream { got } => { write ! (f , "match kind {:?} does not support stream searching" , got ,) } MatchErrorKind :: UnsupportedOverlapping { got } => { write ! (f , "match kind {:?} does not support overlapping searches" , got ,) } MatchErrorKind :: UnsupportedEmpty => { write ! (f , "matching with an empty pattern string is not \
                     supported for this operation" ,) } } } }
};
}
