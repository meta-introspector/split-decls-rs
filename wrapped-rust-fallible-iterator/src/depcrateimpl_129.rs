// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl < I , P > FallibleIterator for TakeWhile < I , P > where I : FallibleIterator , P : FnMut (& I :: Item) -> Result < bool , I :: Error > , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { if self . flag { Ok (None) } else { match self . it . next () ? { Some (item) => { if (self . predicate) (& item) ? { Ok (Some (item)) } else { self . flag = true ; Ok (None) } } None => Ok (None) , } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . flag { (0 , Some (0)) } else { let hint = self . it . size_hint () ; (0 , hint . 1) } } }
};
}
