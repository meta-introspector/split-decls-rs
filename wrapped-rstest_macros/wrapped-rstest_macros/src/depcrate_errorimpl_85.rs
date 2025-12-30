// Generated macro for impl_85 (impl)
macro_rules! Depcrate_errorimpl_85 {
() => {
// Module: crate::error
// Provides: {"impl_85"}
// Dependencies: {}
impl IsImplicitFixture for FixtureInfo { fn is_implicit_fixture (& self , pat : & Pat) -> bool { ! self . data . fixtures () . any (| f | & f . arg == pat) } }
};
}
