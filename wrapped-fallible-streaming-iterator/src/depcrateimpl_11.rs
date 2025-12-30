// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a , I , T , E > DoubleEndedFallibleStreamingIterator for Convert < 'a , I , T > where I : DoubleEndedIterator < Item = Result < & 'a T , E > > , { # [inline] fn advance_back (& mut self) -> Result < () , E > { self . item = match self . it . next_back () { Some (Ok (v)) => Some (v) , Some (Err (e)) => return Err (e) , None => None , } ; Ok (()) } }
};
}
