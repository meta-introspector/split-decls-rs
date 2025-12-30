// Generated macro for impl_4 (impl)
macro_rules! Depcrate_errorimpl_4 {
() => {
// Module: crate::error
// Provides: {"impl_4"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: EmptyChoose => write ! (f , "`arbitrary::Unstructured::choose` must be given a non-empty set of choices") , Error :: NotEnoughData => write ! (f , "There is not enough underlying raw data to construct an `Arbitrary` instance") , Error :: IncorrectFormat => write ! (f , "The raw data is not of the correct format to construct this type") , } } }
};
}
