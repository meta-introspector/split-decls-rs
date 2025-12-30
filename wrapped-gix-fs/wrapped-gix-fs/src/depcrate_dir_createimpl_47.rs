// Generated macro for impl_47 (impl)
macro_rules! Depcrate_dir_createimpl_47 {
() => {
// Module: crate::dir::create
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > Iter < 'a > { fn permanent_failure (& mut self , dir : & 'a Path , err : impl Into < std :: io :: Error > ,) -> Option < Result < & 'a Path , Error < 'a > > > { self . cursors . clear () ; Some (Err (Error :: Permanent { err : err . into () , dir , retries_left : self . retries , retries : self . original_retries , })) } fn intermediate_failure (& self , dir : & 'a Path , err : std :: io :: Error) -> Option < Result < & 'a Path , Error < 'a > > > { Some (Err (Error :: Intermediate { dir , kind : err . kind () })) } }
};
}
