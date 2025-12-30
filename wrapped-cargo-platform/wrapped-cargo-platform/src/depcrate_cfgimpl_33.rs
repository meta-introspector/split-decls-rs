// Generated macro for impl_33 (impl)
macro_rules! Depcrate_cfgimpl_33 {
() => {
// Module: crate::cfg
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a > Token < 'a > { fn classify (& self) -> & 'static str { match * self { Token :: LeftParen => "`(`" , Token :: RightParen => "`)`" , Token :: Ident (..) => "an identifier" , Token :: Comma => "`,`" , Token :: Equals => "`=`" , Token :: String (..) => "a string" , } } }
};
}
