// Generated macro for impl_13 (impl)
macro_rules! Depcrate_databaseimpl_13 {
() => {
// Module: crate::database
// Provides: {"impl_13"}
// Dependencies: {}
impl GrastDb { pub fn new () -> Self { GrastDb { triples : Vec :: new () , index : HashMap :: new () , } } pub fn add_triple (& mut self , subject : & str , predicate : & str , object : & str) { let triple = GrastTriple { subject : subject . to_string () , predicate : predicate . to_string () , object : object . to_string () , } ; let index = self . triples . len () ; self . triples . push (triple) ; self . index . entry (subject . to_string ()) . or_insert_with (Vec :: new) . push (index) ; } pub fn to_turtle (& self) -> String { self . triples . iter () . map (| t | t . to_turtle ()) . collect :: < Vec < _ > > () . join ("\n") } }
};
}
