macro_rules! deps {
    () => {
        Sources!();
        NoPath!();
        MissingIfThisChanged!();
        Ok!();
        Targets!();
    };
}

macro_rules! check_paths {
    () => {
        deps!();
        fn check_paths < 'tcx > (tcx : TyCtxt < 'tcx > , if_this_changed : & Sources , then_this_would_need : & Targets) { if if_this_changed . is_empty () { for & (target_span , _ , _ , _) in then_this_would_need { tcx . dcx () . emit_err (errors :: MissingIfThisChanged { span : target_span }) ; } return ; } tcx . dep_graph . with_query (| query | { for & (_ , source_def_id , ref source_dep_node) in if_this_changed { let dependents = query . transitive_predecessors (source_dep_node) ; for & (target_span , ref target_pass , _ , ref target_dep_node) in then_this_would_need { if ! dependents . contains (& target_dep_node) { tcx . dcx () . emit_err (errors :: NoPath { span : target_span , source : tcx . def_path_str (source_def_id) , target : * target_pass , }) ; } else { tcx . dcx () . emit_err (errors :: Ok { span : target_span }) ; } } } }) ; }
    };
}

check_paths!();