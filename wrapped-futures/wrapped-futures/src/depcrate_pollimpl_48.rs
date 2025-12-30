// Generated macro for impl_48 (impl)
macro_rules! Depcrate_pollimpl_48 {
() => {
// Module: crate::poll
// Provides: {"impl_48"}
// Dependencies: {}
impl < T , E > From < Result < T , E > > for Poll < T , E > { fn from (r : Result < T , E >) -> Poll < T , E > { match r { Ok (t) => Poll :: Ok (t) , Err (t) => Poll :: Err (t) , } } }
};
}
