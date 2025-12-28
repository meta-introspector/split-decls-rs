macro_rules! deps {
    () => {
        NoFragmentCycles!();
        Visitor!();
        CycleDetector!();
        VisitorContext!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for NoFragmentCycles < 'a > { fn exit_document (& mut self , ctx : & mut VisitorContext < 'a > , _doc : & 'a ExecutableDocument) { let mut detector = CycleDetector { visited : HashSet :: new () , spreads : & self . spreads , path_indices : HashMap :: new () , errors : Vec :: new () , } ; for frag in & self . fragment_order { if ! detector . visited . contains (frag) { let mut path = Vec :: new () ; detector . detect_from (frag , & mut path) ; } } ctx . append_errors (detector . errors) ; } fn enter_fragment_definition (& mut self , _ctx : & mut VisitorContext < 'a > , name : & 'a Name , _fragment_definition : & 'a Positioned < FragmentDefinition > ,) { self . current_fragment = Some (name) ; self . fragment_order . push (name) ; } fn exit_fragment_definition (& mut self , _ctx : & mut VisitorContext < 'a > , _name : & 'a Name , _fragment_definition : & 'a Positioned < FragmentDefinition > ,) { self . current_fragment = None ; } fn enter_fragment_spread (& mut self , _ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { if let Some (current_fragment) = self . current_fragment { self . spreads . entry (current_fragment) . or_default () . push ((& fragment_spread . node . fragment_name . node , fragment_spread . pos ,)) ; } } }
    };
}

impl_218!();