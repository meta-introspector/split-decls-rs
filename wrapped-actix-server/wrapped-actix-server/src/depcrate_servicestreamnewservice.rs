// Generated macro for StreamNewService (struct)
macro_rules! Depcrate_serviceStreamNewService {
() => {
// Module: crate::service
// Provides: {"StreamNewService"}
// Dependencies: {}
pub (crate) struct StreamNewService < F : ServerServiceFactory < Io > , Io : FromStream > { name : String , inner : F , token : usize , addr : SocketAddr , _t : PhantomData < Io > , }
};
}
