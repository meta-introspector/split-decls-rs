// Generated macro for impl_111 (impl)
macro_rules! Depcrate_dfa_onepassimpl_111 {
() => {
// Module: crate::dfa::onepass
// Provides: {"impl_111"}
// Dependencies: {}
impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { use self :: BuildErrorKind :: * ; match self . kind { NFA (_) => write ! (f , "error building NFA") , Word (_) => write ! (f , "NFA contains Unicode word boundary") , TooManyStates { limit } => write ! (f , "one-pass DFA exceeded a limit of {limit:?} \
                 for number of states" ,) , TooManyPatterns { limit } => write ! (f , "one-pass DFA exceeded a limit of {limit:?} \
                 for number of patterns" ,) , UnsupportedLook { look } => write ! (f , "one-pass DFA does not support the {look:?} assertion" ,) , ExceededSizeLimit { limit } => write ! (f , "one-pass DFA exceeded size limit of {limit:?} during building" ,) , NotOnePass { msg } => write ! (f , "one-pass DFA could not be built because \
                 pattern is not one-pass: {}" , msg ,) , } } }
};
}
