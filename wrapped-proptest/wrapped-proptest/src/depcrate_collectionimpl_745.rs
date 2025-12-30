// Generated macro for impl_745 (impl)
macro_rules! Depcrate_collectionimpl_745 {
() => {
// Module: crate::collection
// Provides: {"impl_745"}
// Dependencies: {}
impl < T : ValueTree > ValueTree for VecValueTree < T > { type Value = Vec < T :: Value > ; fn current (& self) -> Vec < T :: Value > { self . elements . iter () . enumerate () . filter (| & (ix , _) | self . included_elements . test (ix)) . map (| (_ , element) | element . current ()) . collect () } fn simplify (& mut self) -> bool { if let Shrink :: DeleteElement (ix) = self . shrink { if ix >= self . elements . len () || self . included_elements . count () == self . min_size { self . shrink = Shrink :: ShrinkElement (0) ; } else { self . included_elements . clear (ix) ; self . prev_shrink = Some (self . shrink) ; self . shrink = Shrink :: DeleteElement (ix + 1) ; return true ; } } while let Shrink :: ShrinkElement (ix) = self . shrink { if ix >= self . elements . len () { return false ; } if ! self . included_elements . test (ix) { self . shrink = Shrink :: ShrinkElement (ix + 1) ; continue ; } if ! self . elements [ix] . simplify () { self . shrink = Shrink :: ShrinkElement (ix + 1) ; } else { self . prev_shrink = Some (self . shrink) ; return true ; } } panic ! ("Unexpected shrink state") ; } fn complicate (& mut self) -> bool { match self . prev_shrink { None => false , Some (Shrink :: DeleteElement (ix)) => { self . included_elements . set (ix) ; self . prev_shrink = None ; true } Some (Shrink :: ShrinkElement (ix)) => { if self . elements [ix] . complicate () { true } else { self . prev_shrink = None ; false } } } } }
};
}
