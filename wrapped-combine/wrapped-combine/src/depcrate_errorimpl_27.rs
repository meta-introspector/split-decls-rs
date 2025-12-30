// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorimpl_27 {
() => {
// Module: crate::error
// Provides: {"impl_27"}
// Dependencies: {}
impl < 's , T , R , F > ErrorInfo < 's , T , R > for Info < T , R , F > where T : Clone , R : Clone , F : fmt :: Display + 's , { type Format = & 's F ; fn into_info (& 's self) -> Info < T , R , < Self as ErrorInfo < '_ , T , R > > :: Format > { match self { Info :: Token (b) => Info :: Token (b . clone ()) , Info :: Range (b) => Info :: Range (b . clone ()) , Info :: Static (b) => Info :: Static (* b) , Info :: Format (b) => Info :: Format (b) , } } }
};
}
