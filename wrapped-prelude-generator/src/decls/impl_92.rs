macro_rules! deps {
    () => {
        ImplLatticeInfo!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl ImplLatticeInfo { pub fn new (impl_for_type : String) -> Self { ImplLatticeInfo { impl_for_type , method_co_occurrences : HashMap :: new () , total_expressions_analyzed : 0 , } } pub fn add_co_occurrence (& mut self , method_names : BTreeSet < String >) { let key = method_names . into_iter () . collect :: < Vec < String > > () . join ("::") ; * self . method_co_occurrences . entry (key) . or_insert (0) += 1 ; self . total_expressions_analyzed += 1 ; } }
    };
}

impl_92!();