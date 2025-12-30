// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl < T , F , B > FallibleIterator for Map < T , F > where T : FallibleIterator , F : FnMut (T :: Item) -> Result < B , T :: Error > , { type Item = B ; type Error = T :: Error ; # [inline] fn next (& mut self) -> Result < Option < B > , T :: Error > { match self . it . next () { Ok (Some (v)) => Ok (Some ((self . f) (v) ?)) , Ok (None) => Ok (None) , Err (e) => Err (e) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn try_fold < C , E , G > (& mut self , init : C , mut f : G) -> Result < C , E > where E : From < T :: Error > , G : FnMut (C , B) -> Result < C , E > , { let map = & mut self . f ; self . it . try_fold (init , | b , v | f (b , map (v) ?)) } }
};
}
