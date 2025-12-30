// Generated macro for impl_862 (impl)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedimpl_862 {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"impl_862"}
// Dependencies: {}
impl < K : Eq + Hash + Clone , V > OrderedMap < K , V > { fn new () -> OrderedMap < K , V > { OrderedMap { data : HashMap :: new () , insert_order : Vec :: new () , } } fn iter (& self) -> OrderedMapIter < '_ , K , V > { OrderedMapIter { map : & self . data , inner : self . insert_order . iter () , } } fn get < Q > (& self , k : & Q) -> Option < & V > where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { self . data . get (k) } fn get_mut < Q > (& mut self , k : & Q) -> Option < & mut V > where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { self . data . get_mut (k) } fn contains_key < Q > (& self , k : & Q) -> bool where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { self . data . contains_key (k) } fn insert (& mut self , k : K , v : V) -> Option < V > { let result = self . data . insert (k . clone () , v) ; if result . is_none () { self . insert_order . push (k) ; } result } }
};
}
