// Generated macro for impl_199 (impl)
macro_rules! Depcrate_dfa_automatonimpl_199 {
() => {
// Module: crate::dfa::automaton
// Provides: {"impl_199"}
// Dependencies: {}
impl core :: fmt :: Display for StartError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { StartError :: Quit { byte } => write ! (f , "error computing start state because the look-behind byte \
                 {:?} triggered a quit state" , crate :: util :: escape :: DebugByte (byte) ,) , StartError :: UnsupportedAnchored { mode : Anchored :: Yes } => { write ! (f , "error computing start state because \
                     anchored searches are not supported or enabled") } StartError :: UnsupportedAnchored { mode : Anchored :: No } => { write ! (f , "error computing start state because \
                     unanchored searches are not supported or enabled") } StartError :: UnsupportedAnchored { mode : Anchored :: Pattern (pid) , } => { write ! (f , "error computing start state because \
                     anchored searches for a specific pattern ({}) \
                     are not supported or enabled" , pid . as_usize () ,) } } } }
};
}
