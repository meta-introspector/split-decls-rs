// Generated macro for impl_328 (impl)
macro_rules! Depcrate_builder_styled_strimpl_328 {
() => {
// Module: crate::builder::styled_str
// Provides: {"impl_328"}
// Dependencies: {}
impl From < Cow < 'static , str > > for StyledStr { fn from (cow : Cow < 'static , str >) -> Self { match cow { Cow :: Borrowed (s) => StyledStr :: from (s) , Cow :: Owned (s) => StyledStr :: from (s) , } } }
};
}
