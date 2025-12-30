// Generated macro for impl_168 (impl)
macro_rules! Depcrate_adaptorsimpl_168 {
() => {
// Module: crate::adaptors
// Provides: {"impl_168"}
// Dependencies: {}
impl < I , F , T , E > Iterator for FilterOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (& T) -> bool , { type Item = Result < T , E > ; fn next (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . find (| res | match res { Ok (t) => f (t) , _ => true , }) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } fn fold < Acc , Fold > (self , init : Acc , fold_f : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { let mut f = self . f ; self . iter . filter (| v | v . as_ref () . map (& mut f) . unwrap_or (true)) . fold (init , fold_f) } fn collect < C > (self) -> C where C : FromIterator < Self :: Item > , { let mut f = self . f ; self . iter . filter (| v | v . as_ref () . map (& mut f) . unwrap_or (true)) . collect () } }
};
}
