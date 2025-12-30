// Generated macro for impl_23 (impl)
macro_rules! Depcrate_adt_mapimpl_23 {
() => {
// Module: crate::adt::map
// Provides: {"impl_23"}
// Dependencies: {}
impl < K : PartialEq + Clone + Hash + Eq , V : Clone > ScopedMap < K , V > { pub fn new () -> Self { ScopedMap { stack : Vec :: new () } } pub fn push (& mut self) { self . stack . push (Vec :: new ()) ; } pub fn pop (& mut self) { if ! self . is_empty () { self . stack . pop () ; } } pub fn len (& self) -> usize { self . stack . len () } pub fn is_empty (& self) -> bool { self . stack . is_empty () } pub fn insert (& mut self , key : & K , val : & V) { assert ! (! self . is_empty ()) ; let scope = self . stack . last_mut () . unwrap () ; for pair in scope { if pair . 0 == * key { pair . 1 = val . clone () ; return ; } } self . stack . last_mut () . unwrap () . push ((key . clone () , val . clone ())) ; } pub fn flatten (& self) -> HashMap < K , V > { let mut map : HashMap < K , V > = HashMap :: new () ; for scope in self . stack . iter () { for pair in scope . iter () { map . insert (pair . 0 . clone () , pair . 1 . clone ()) ; } } map } pub fn get (& self , key : & K) -> Option < V > { for scope in self . stack . iter () . rev () { for pair in scope { if pair . 0 == * key { return Option :: Some (pair . 1 . clone ()) ; } } } Option :: None } pub fn has (& self , key : & K) -> bool { matches ! (self . get (key) , Option :: Some (_)) } }
};
}
