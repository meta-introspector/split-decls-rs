// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl Parser { # [inline] pub fn can_range () -> Self { Parser { can_range : true , } } # [inline] pub fn cannot_range () -> Self { Parser { can_range : false , } } pub fn parse (& self , src : impl Parse) -> Result < Mime , ParseError > { rfc7231 :: parse (self , src) } }
};
}
