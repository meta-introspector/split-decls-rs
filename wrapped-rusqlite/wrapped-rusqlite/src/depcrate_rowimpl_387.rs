// Generated macro for impl_387 (impl)
macro_rules! Depcrate_rowimpl_387 {
() => {
// Module: crate::row
// Provides: {"impl_387"}
// Dependencies: {}
impl < F , B > FallibleIterator for Map < '_ , F > where F : FnMut (& Row < '_ >) -> Result < B > , { type Error = Error ; type Item = B ; # [inline] fn next (& mut self) -> Result < Option < B > > { match self . rows . next () ? { Some (v) => Ok (Some ((self . f) (v) ?)) , None => Ok (None) , } } }
};
}
