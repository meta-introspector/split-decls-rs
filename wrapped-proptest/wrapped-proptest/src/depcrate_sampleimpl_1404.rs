// Generated macro for impl_1404 (impl)
macro_rules! Depcrate_sampleimpl_1404 {
() => {
// Module: crate::sample
// Provides: {"impl_1404"}
// Dependencies: {}
impl Selector { # [doc = " Pick a random element from iterable `it`."] # [doc = ""] # [doc = " The selection is unaffected by the elements themselves, and is"] # [doc = " dependent only on the actual length of `it`."] # [doc = ""] # [doc = " `it` is always iterated completely."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " Panics if `it` has no elements."] pub fn select < T : IntoIterator > (& self , it : T) -> T :: Item { self . try_select (it) . expect ("select from empty iterator") } # [doc = " Pick a random element from iterable `it`."] # [doc = ""] # [doc = " Returns `None` if `it` is empty."] # [doc = ""] # [doc = " The selection is unaffected by the elements themselves, and is"] # [doc = " dependent only on the actual length of `it`."] # [doc = ""] # [doc = " `it` is always iterated completely."] pub fn try_select < T : IntoIterator > (& self , it : T) -> Option < T :: Item > { let mut bias = 0u64 ; let mut min_score = 0 ; let mut best = None ; let mut rng = self . rng . clone () ; for item in it { let score = bias . saturating_add (rng . random ()) ; if best . is_none () || score < min_score { best = Some (item) ; min_score = score ; } bias = bias . saturating_add (self . bias_increment) ; } best } }
};
}
