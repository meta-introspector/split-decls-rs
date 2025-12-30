// Generated macro for impl_74 (impl)
macro_rules! Depcrate_serviceimpl_74 {
() => {
// Module: crate::service
// Provides: {"impl_74"}
// Dependencies: {}
impl < F , Io > StreamNewService < F , Io > where F : ServerServiceFactory < Io > , Io : FromStream + Send + 'static , { pub (crate) fn create (name : String , token : usize , inner : F , addr : SocketAddr ,) -> Box < dyn InternalServiceFactory > { Box :: new (Self { name , token , inner , addr , _t : PhantomData , }) } }
};
}
