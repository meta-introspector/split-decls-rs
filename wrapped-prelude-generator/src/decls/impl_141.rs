macro_rules! deps {
    () => {
        StructLatticeInfo!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl StructLatticeInfo { pub fn new (struct_name : String) -> Self { StructLatticeInfo { struct_name , field_co_occurrences : HashMap :: new () , total_expressions_analyzed : 0 , } } pub fn add_co_occurrence (& mut self , field_types : BTreeSet < String >) { let key = field_types . into_iter () . collect :: < Vec < String > > () . join ("::") ; * self . field_co_occurrences . entry (key) . or_insert (0) += 1 ; self . total_expressions_analyzed += 1 ; } }
    };
}

impl_141!();