// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , SomePart : Part , Reference : PartialRef < 'a > > HasTarget for Mut < SomePart , Reference > where Reference :: Target : HasPart < SomePart > , { type Target = Reference :: Target ; }
};
}
