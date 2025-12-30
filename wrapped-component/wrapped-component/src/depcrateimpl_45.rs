// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl IActivationFactory_Impl for ClassFactory_Impl { fn ActivateInstance (& self) -> Result < IInspectable > { Ok (Class (RwLock :: new (0)) . into ()) } }
};
}
