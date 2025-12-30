// Generated macro for impl_75 (impl)
macro_rules! Depcrate_serviceimpl_75 {
() => {
// Module: crate::service
// Provides: {"impl_75"}
// Dependencies: {}
impl < F , Io > InternalServiceFactory for StreamNewService < F , Io > where F : ServerServiceFactory < Io > , Io : FromStream + Send + 'static , { fn name (& self , _ : usize) -> & str { & self . name } fn clone_factory (& self) -> Box < dyn InternalServiceFactory > { Box :: new (Self { name : self . name . clone () , inner : self . inner . clone () , token : self . token , addr : self . addr , _t : PhantomData , }) } fn create (& self) -> LocalBoxFuture < 'static , Result < (usize , BoxedServerService) , () > > { let token = self . token ; let fut = self . inner . create () . new_service (()) ; Box :: pin (async move { match fut . await { Ok (inner) => { let service = Box :: new (StreamService :: new (inner)) as _ ; Ok ((token , service)) } Err (_) => Err (()) , } }) } }
};
}
