// Generated macro for impl_76 (impl)
macro_rules! Depcrate_serviceimpl_76 {
() => {
// Module: crate::service
// Provides: {"impl_76"}
// Dependencies: {}
impl < F , T , I > ServerServiceFactory < I > for F where F : Fn () -> T + Send + Clone + 'static , T : BaseServiceFactory < I , Config = () > , I : FromStream , { type Factory = T ; fn create (& self) -> T { (self) () } }
};
}
