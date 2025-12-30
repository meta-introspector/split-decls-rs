// Generated macro for lock (module)
macro_rules! Depcrate_synclock {
() => {
// Module: crate::sync
// Provides: {"lock"}
// Dependencies: {}
# [cfg (not (threadsafe))] mod lock { use std :: cell :: { RefCell , RefMut } ; use std :: rc :: Rc ; # [doc = " Single threaded lock: a `RefCell` so we should safely panic if somehow"] # [doc = " trying to access the stored data twice from the same thread."] pub (crate) struct Lock < A > { lock : Rc < RefCell < A > > , } impl < A > Lock < A > { pub (crate) fn new (value : A) -> Self { Lock { lock : Rc :: new (RefCell :: new (value)) , } } # [inline] pub (crate) fn lock (& mut self) -> Option < RefMut < '_ , A > > { self . lock . try_borrow_mut () . ok () } } impl < A > Clone for Lock < A > { fn clone (& self) -> Self { Lock { lock : self . lock . clone () , } } } }
};
}
