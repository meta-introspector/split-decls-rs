// Generated macro for impl_15 (impl)
macro_rules! Depcrate_aioimpl_15 {
() => {
// Module: crate::aio
// Provides: {"impl_15"}
// Dependencies: {}
impl < T : Aio > event :: Source for Source < T > { fn register (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { assert ! (interests . is_aio ()) ; let udata = usize :: from (token) ; let kq = registry . as_raw_fd () ; self . _register_raw (kq , udata) ; Ok (()) } fn reregister (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { self . register (registry , token , interests) } fn deregister (& mut self , _registry : & Registry) -> io :: Result < () > { self . _deregister_raw () ; Ok (()) } }
};
}
