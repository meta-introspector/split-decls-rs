macro_rules! deps {
    () => {
        ClosureOutlivesSubject!();
        ClosureRegionRequirements!();
    };
}

macro_rules! for_each_region_constraint {
    () => {
        deps!();
        fn for_each_region_constraint < 'tcx > (tcx : TyCtxt < 'tcx > , closure_region_requirements : & ClosureRegionRequirements < 'tcx > , with_msg : & mut dyn FnMut (String) -> io :: Result < () > ,) -> io :: Result < () > { for req in & closure_region_requirements . outlives_requirements { let subject = match req . subject { ClosureOutlivesSubject :: Region (subject) => format ! ("{subject:?}") , ClosureOutlivesSubject :: Ty (ty) => { with_no_trimmed_paths ! (format ! ("{}" , ty . instantiate (tcx , | vid | ty :: Region :: new_var (tcx , vid)))) } } ; with_msg (format ! ("where {}: {:?}" , subject , req . outlived_free_region ,)) ? ; } Ok (()) }
    };
}

for_each_region_constraint!()