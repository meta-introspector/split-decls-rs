// Generated macro for Blocking (struct)
macro_rules! Depcrate_waiter_blockingBlocking {
() => {
// Module: crate::waiter::blocking
// Provides: {"Blocking"}
// Dependencies: {}
# [doc = " Blocking implements a reading operation on a different thread."] # [doc = " It stops the thread once any [`Error`] is encountered."] # [derive (Debug)] pub struct Blocking < R > { _id : usize , _thread : JoinHandle < () > , _reader : PhantomData < R > , }
};
}
