// Generated macro for impl_165 (impl)
macro_rules! Depcrate_dfa_sparseimpl_165 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a > fmt :: Debug for State < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut printed = false ; for i in 0 .. (self . ntrans - 1) { let next = self . next_at (i) ; if next == DEAD { continue ; } if printed { write ! (f , ", ") ? ; } let (start , end) = self . range (i) ; if start == end { write ! (f , "{:?} => {:?}" , DebugByte (start) , next . as_usize ()) ? ; } else { write ! (f , "{:?}-{:?} => {:?}" , DebugByte (start) , DebugByte (end) , next . as_usize () ,) ? ; } printed = true ; } let eoi = self . next_at (self . ntrans - 1) ; if eoi != DEAD { if printed { write ! (f , ", ") ? ; } write ! (f , "EOI => {:?}" , eoi . as_usize ()) ? ; } Ok (()) } }
};
}
