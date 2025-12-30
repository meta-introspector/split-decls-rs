// Generated macro for impl_86 (impl)
macro_rules! Depcrate_errorimpl_86 {
() => {
// Module: crate::error
// Provides: {"impl_86"}
// Dependencies: {}
impl IsImplicitFixture for RsTestInfo { fn is_implicit_fixture (& self , pat : & Pat) -> bool { ! self . data . case_args () . any (| a | a == pat) && ! self . data . list_values () . any (| a | & a . arg == pat) && ! self . data . fixtures () . any (| f | & f . arg == pat) } }
};
}
