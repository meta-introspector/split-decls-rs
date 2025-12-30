// Generated macro for impl_35 (impl)
macro_rules! Depcrate_deriveimpl_35 {
() => {
// Module: crate::derive
// Provides: {"impl_35"}
// Dependencies: {}
impl < T : Parser > Parser for Box < T > { fn parse () -> Self { Box :: new (< T as Parser > :: parse ()) } fn try_parse () -> Result < Self , Error > { < T as Parser > :: try_parse () . map (Box :: new) } fn parse_from < I , It > (itr : I) -> Self where I : IntoIterator < Item = It > , It : Into < OsString > + Clone , { Box :: new (< T as Parser > :: parse_from (itr)) } fn try_parse_from < I , It > (itr : I) -> Result < Self , Error > where I : IntoIterator < Item = It > , It : Into < OsString > + Clone , { < T as Parser > :: try_parse_from (itr) . map (Box :: new) } }
};
}
