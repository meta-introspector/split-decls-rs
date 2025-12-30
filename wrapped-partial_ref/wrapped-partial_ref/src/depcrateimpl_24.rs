// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , SomePart : Part , Reference : PartialRef < 'a > > HasTarget for Const < SomePart , Reference > where Reference :: Target : HasPart < SomePart > , { type Target = Reference :: Target ; }
};
}
