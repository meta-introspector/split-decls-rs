// Generated macro for impl_413 (impl)
macro_rules! Depcrate_mir_transitive_relationimpl_413 {
() => {
// Module: crate::mir::transitive_relation
// Provides: {"impl_413"}
// Dependencies: {}
impl TransitiveRelation { pub fn add (& mut self , a : mir :: Local , b : mir :: Local) { self . relations . entry (a) . or_default () . push (b) ; } pub fn reachable_from (& self , a : mir :: Local , domain_size : usize) -> DenseBitSet < mir :: Local > { let mut seen = DenseBitSet :: new_empty (domain_size) ; let mut stack = vec ! [a] ; while let Some (u) = stack . pop () { if let Some (edges) = self . relations . get (& u) { for & v in edges { if seen . insert (v) { stack . push (v) ; } } } } seen } }
};
}
