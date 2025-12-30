// Generated macro for impl_99 (impl)
macro_rules! Depcrate_socketimpl_99 {
() => {
// Module: crate::socket
// Provides: {"impl_99"}
// Dependencies: {}
impl Source for MioListener { fn register (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { match * self { MioListener :: Tcp (ref mut lst) => lst . register (registry , token , interests) , # [cfg (unix)] MioListener :: Uds (ref mut lst) => lst . register (registry , token , interests) , } } fn reregister (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { match * self { MioListener :: Tcp (ref mut lst) => lst . reregister (registry , token , interests) , # [cfg (unix)] MioListener :: Uds (ref mut lst) => lst . reregister (registry , token , interests) , } } fn deregister (& mut self , registry : & Registry) -> io :: Result < () > { match * self { MioListener :: Tcp (ref mut lst) => lst . deregister (registry) , # [cfg (unix)] MioListener :: Uds (ref mut lst) => { let res = lst . deregister (registry) ; if let Ok (addr) = lst . local_addr () { if let Some (path) = addr . as_pathname () { let _ = std :: fs :: remove_file (path) ; } } res } } } }
};
}
