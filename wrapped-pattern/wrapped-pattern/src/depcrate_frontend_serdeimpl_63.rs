// Generated macro for impl_63 (impl)
macro_rules! Depcrate_frontend_serdeimpl_63 {
() => {
// Module: crate::frontend::serde
// Provides: {"impl_63"}
// Dependencies: {}
impl < B : PatternBackend > Default for PatternString < B > where Box < B :: Store > : for < 'a > From < & 'a B :: Store > , { fn default () -> Self { Self (Box :: < Pattern < B > > :: default ()) } }
};
}
