// Generated macro for impl_78 (impl)
macro_rules! Depcrate_waiter_blockingimpl_78 {
() => {
// Module: crate::waiter::blocking
// Provides: {"impl_78"}
// Dependencies: {}
impl < R > Blocking < R > { # [doc = " Creates a new blocking reader, spawning a new thread for it."] pub fn new (id : usize , mut reader : R , sendr : Sender < (usize , Result < Option < u8 > >) >) -> Self where R : Read + Send + 'static , { let handle = std :: thread :: spawn (move | | { let mut buffer = Vec :: new () ; let mut buf = [0 ; 1] ; loop { match reader . read (& mut buf) { Ok (n) => { if ! buffer . is_empty () { for b in buffer . drain (..) . collect :: < Vec < _ > > () { try_send (id , Ok (Some (b)) , & sendr , & mut buffer) ; } } if n == 0 { try_send (id , Ok (None) , & sendr , & mut buffer) ; break ; } else { try_send (id , Ok (Some (buf [0])) , & sendr , & mut buffer) ; } } Err (err) => { try_send (id , Err (err) , & sendr , & mut buffer) ; break ; } } } }) ; Self { _id : id , _thread : handle , _reader : PhantomData , } } pub fn join (self) -> std :: thread :: Result < () > { self . _thread . join () } }
};
}
