macro_rules! deps {
    () => {
        GrastTriple!();
        GrastDb!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl GrastDb { pub fn new () -> Self { GrastDb { triples : Vec :: new () , index : HashMap :: new () , } } pub fn add_triple (& mut self , subject : & str , predicate : & str , object : & str) { let triple = GrastTriple { subject : subject . to_string () , predicate : predicate . to_string () , object : object . to_string () , } ; let index = self . triples . len () ; self . triples . push (triple) ; self . index . entry (subject . to_string ()) . or_insert_with (Vec :: new) . push (index) ; } pub fn to_turtle (& self) -> String { self . triples . iter () . map (| t | t . to_turtle ()) . collect :: < Vec < _ > > () . join ("\n") } }
    };
}

impl_7!()