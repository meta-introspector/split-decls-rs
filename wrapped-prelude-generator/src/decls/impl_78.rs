macro_rules! deps {
    () => {
        EnumLatticeInfo!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl EnumLatticeInfo { pub fn new (enum_name : String) -> Self { EnumLatticeInfo { enum_name , variant_type_co_occurrences : HashMap :: new () , total_expressions_analyzed : 0 , } } pub fn add_co_occurrence (& mut self , variant_types : BTreeSet < String >) { let key = variant_types . into_iter () . collect :: < Vec < String > > () . join ("::") ; * self . variant_type_co_occurrences . entry (key) . or_insert (0) += 1 ; self . total_expressions_analyzed += 1 ; } }
    };
}

impl_78!()