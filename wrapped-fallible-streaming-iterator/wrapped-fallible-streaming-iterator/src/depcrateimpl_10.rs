// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a , I , T , E > FallibleStreamingIterator for Convert < 'a , I , T > where I : Iterator < Item = Result < & 'a T , E > > , { type Item = T ; type Error = E ; # [inline] fn advance (& mut self) -> Result < () , E > { self . item = match self . it . next () { Some (Ok (v)) => Some (v) , Some (Err (e)) => return Err (e) , None => None , } ; Ok (()) } # [inline] fn get (& self) -> Option < & T > { self . item } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
