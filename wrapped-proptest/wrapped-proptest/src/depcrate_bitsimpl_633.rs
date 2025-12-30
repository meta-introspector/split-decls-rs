// Generated macro for impl_633 (impl)
macro_rules! Depcrate_bitsimpl_633 {
() => {
// Module: crate::bits
// Provides: {"impl_633"}
// Dependencies: {}
impl < T : BitSetLike > ValueTree for BitSetValueTree < T > { type Value = T ; fn current (& self) -> T { self . inner . clone () } fn simplify (& mut self) -> bool { if self . inner . count () <= self . min_count { return false ; } while self . shrink < self . inner . len () && ! self . inner . test (self . shrink) { self . shrink += 1 ; } if self . shrink >= self . inner . len () { self . prev_shrink = None ; false } else { self . prev_shrink = Some (self . shrink) ; self . inner . clear (self . shrink) ; self . shrink += 1 ; true } } fn complicate (& mut self) -> bool { if let Some (bit) = self . prev_shrink . take () { self . inner . set (bit) ; true } else { false } } }
};
}
