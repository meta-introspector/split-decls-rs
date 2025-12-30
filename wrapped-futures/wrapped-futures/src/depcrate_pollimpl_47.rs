// Generated macro for impl_47 (impl)
macro_rules! Depcrate_pollimpl_47 {
() => {
// Module: crate::poll
// Provides: {"impl_47"}
// Dependencies: {}
impl < T , E > Poll < T , E > { # [doc = " Change the success type of this `Poll` value with the closure provided"] pub fn map < F , U > (self , f : F) -> Poll < U , E > where F : FnOnce (T) -> U { match self { Poll :: NotReady => Poll :: NotReady , Poll :: Ok (t) => Poll :: Ok (f (t)) , Poll :: Err (e) => Poll :: Err (e) , } } # [doc = " Change the error type of this `Poll` value with the closure provided"] pub fn map_err < F , U > (self , f : F) -> Poll < T , U > where F : FnOnce (E) -> U { match self { Poll :: NotReady => Poll :: NotReady , Poll :: Ok (t) => Poll :: Ok (t) , Poll :: Err (e) => Poll :: Err (f (e)) , } } # [doc = " Returns whether this is `Poll::NotReady`"] pub fn is_not_ready (& self) -> bool { match * self { Poll :: NotReady => true , _ => false , } } # [doc = " Returns whether this is either `Poll::Ok` or `Poll::Err`"] pub fn is_ready (& self) -> bool { ! self . is_not_ready () } # [doc = " Unwraps this `Poll` into a `Result`, panicking if it's not ready."] pub fn unwrap (self) -> Result < T , E > { match self { Poll :: Ok (t) => Ok (t) , Poll :: Err (t) => Err (t) , Poll :: NotReady => panic ! ("unwrapping a Poll that wasn't ready") , } } }
};
}
