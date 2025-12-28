macro_rules! deps {
    () => {
        CollectMemberConstraintsVisitor!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < 'tcx > CollectMemberConstraintsVisitor < '_ , '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . rcx . infcx . tcx } fn visit_closure_args (& mut self , def_id : DefId , args : GenericArgsRef < 'tcx >) { let generics = self . cx () . generics_of (def_id) ; for arg in args . iter () . skip (generics . parent_count) { arg . visit_with (self) ; } } }
    };
}

impl_330!()