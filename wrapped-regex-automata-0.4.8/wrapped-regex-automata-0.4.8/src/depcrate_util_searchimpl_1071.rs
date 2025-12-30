// Generated macro for impl_1071 (impl)
macro_rules! Depcrate_util_searchimpl_1071 {
() => {
// Module: crate::util::search
// Provides: {"impl_1071"}
// Dependencies: {}
impl core :: fmt :: Display for MatchError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match * self . kind () { MatchErrorKind :: Quit { byte , offset } => write ! (f , "quit search after observing byte {:?} at offset {}" , DebugByte (byte) , offset ,) , MatchErrorKind :: GaveUp { offset } => { write ! (f , "gave up searching at offset {}" , offset) } MatchErrorKind :: HaystackTooLong { len } => { write ! (f , "haystack of length {} is too long" , len) } MatchErrorKind :: UnsupportedAnchored { mode : Anchored :: Yes } => { write ! (f , "anchored searches are not supported or enabled") } MatchErrorKind :: UnsupportedAnchored { mode : Anchored :: No } => { write ! (f , "unanchored searches are not supported or enabled") } MatchErrorKind :: UnsupportedAnchored { mode : Anchored :: Pattern (pid) , } => { write ! (f , "anchored searches for a specific pattern ({}) are \
                     not supported or enabled" , pid . as_usize () ,) } } } }
};
}
