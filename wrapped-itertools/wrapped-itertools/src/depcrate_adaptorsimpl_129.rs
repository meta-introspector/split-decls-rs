// Generated macro for impl_129 (impl)
macro_rules! Depcrate_adaptorsimpl_129 {
() => {
// Module: crate::adaptors
// Provides: {"impl_129"}
// Dependencies: {}
impl < I , J > Iterator for Product < I , J > where I : Iterator , J : Clone + Iterator , I :: Item : Clone , { type Item = (I :: Item , J :: Item) ; fn next (& mut self) -> Option < Self :: Item > { let Self { a , a_cur , b , b_orig , } = self ; let elt_b = match b . next () { None => { * b = b_orig . clone () ; match b . next () { None => return None , Some (x) => { * a_cur = Some (a . next ()) ; x } } } Some (x) => x , } ; a_cur . get_or_insert_with (| | a . next ()) . as_ref () . map (| a | (a . clone () , elt_b)) } fn size_hint (& self) -> (usize , Option < usize >) { let mut sh = size_hint :: mul (self . a . size_hint () , self . b_orig . size_hint ()) ; if matches ! (self . a_cur , Some (Some (_))) { sh = size_hint :: add (sh , self . b . size_hint ()) ; } sh } fn fold < Acc , G > (self , mut accum : Acc , mut f : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { let Self { mut a , a_cur , mut b , b_orig , } = self ; if let Some (mut elt_a) = a_cur . unwrap_or_else (| | a . next ()) { loop { accum = b . fold (accum , | acc , elt | f (acc , (elt_a . clone () , elt))) ; if let Some (next_elt_a) = a . next () { b = b_orig . clone () ; elt_a = next_elt_a ; } else { break ; } } } accum } }
};
}
