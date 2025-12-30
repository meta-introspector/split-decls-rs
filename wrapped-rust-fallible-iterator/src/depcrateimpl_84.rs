// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl < I > FallibleIterator for Enumerate < I > where I : FallibleIterator , { type Item = (usize , I :: Item) ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < (usize , I :: Item) > , I :: Error > { self . it . next () . map (| o | { o . map (| e | { let i = self . n ; self . n += 1 ; (i , e) }) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn count (self) -> Result < usize , I :: Error > { self . it . count () } # [inline] fn nth (& mut self , n : usize) -> Result < Option < (usize , I :: Item) > , I :: Error > { match self . it . nth (n) ? { Some (v) => { let i = self . n + n ; self . n = i + 1 ; Ok (Some ((i , v))) } None => Ok (None) , } } # [inline] fn try_fold < B , E , F > (& mut self , init : B , mut f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , (usize , I :: Item)) -> Result < B , E > , { let n = & mut self . n ; self . it . try_fold (init , | acc , v | { let i = * n ; * n += 1 ; f (acc , (i , v)) }) } }
};
}
