// Generated macro for impl_158 (impl)
macro_rules! Depcrate_stream_easyimpl_158 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_158"}
// Dependencies: {}
impl < T , R , F > From < PrimitiveInfo < T , R , F > > for Info < T , R > where F : fmt :: Display , { fn from (info : PrimitiveInfo < T , R , F >) -> Self { match info { PrimitiveInfo :: Token (b) => Info :: Token (b) , PrimitiveInfo :: Range (b) => Info :: Range (b) , PrimitiveInfo :: Static (b) => Info :: Static (b) , PrimitiveInfo :: Format (b) => Info :: Owned (b . to_string ()) , } } }
};
}
