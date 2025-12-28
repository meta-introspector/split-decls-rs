macro_rules! deps {
    () => {
        CheckAlignment!();
        CanAccessMutGlobal!();
        CompileTimeMachine!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'tcx > CompileTimeMachine < 'tcx > { pub (crate) fn new (can_access_mut_global : CanAccessMutGlobal , check_alignment : CheckAlignment ,) -> Self { CompileTimeMachine { num_evaluated_steps : 0 , stack : Vec :: new () , can_access_mut_global , check_alignment , static_root_ids : None , union_data_ranges : FxHashMap :: default () , } } }
    };
}

impl_121!();