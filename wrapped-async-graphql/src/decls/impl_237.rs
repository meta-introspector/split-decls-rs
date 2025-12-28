macro_rules! deps {
    () => {
        Visitor!();
        FindConflicts!();
        OverlappingFieldsCanBeMerged!();
        VisitorContext!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for OverlappingFieldsCanBeMerged { fn enter_selection_set (& mut self , ctx : & mut VisitorContext < 'a > , selection_set : & 'a Positioned < SelectionSet > ,) { let mut find_conflicts = FindConflicts { outputs : Default :: default () , visited : Default :: default () , ctx , } ; find_conflicts . find (None , selection_set) ; } }
    };
}

impl_237!()