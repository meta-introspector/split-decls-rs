macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'tcx > CoverageInfoBuilderMethods < 'tcx > for Builder < '_ , '_ , 'tcx > { # [instrument (level = "debug" , skip (self))] fn add_coverage (& mut self , instance : Instance < 'tcx > , kind : & CoverageKind) { let bx = self ; let Some (_coverage_cx) = & bx . cx . coverage_cx else { return } ; let Some (function_coverage_info) = bx . tcx . instance_mir (instance . def) . function_coverage_info . as_deref () else { debug ! ("function has a coverage statement but no coverage info") ; return ; } ; let Some (ids_info) = bx . tcx . coverage_ids_info (instance . def) else { debug ! ("function has a coverage statement but no IDs info") ; return ; } ; match * kind { CoverageKind :: SpanMarker | CoverageKind :: BlockMarker { .. } => unreachable ! ("marker statement {kind:?} should have been removed by CleanupPostBorrowck") , CoverageKind :: VirtualCounter { bcb } if let Some (& id) = ids_info . phys_counter_for_node . get (& bcb) => { let fn_name = bx . ensure_pgo_func_name_var (instance) ; let hash = bx . const_u64 (function_coverage_info . function_source_hash) ; let num_counters = bx . const_u32 (ids_info . num_counters) ; let index = bx . const_u32 (id . as_u32 ()) ; debug ! ("codegen intrinsic instrprof.increment(fn_name={:?}, hash={:?}, num_counters={:?}, index={:?})" , fn_name , hash , num_counters , index ,) ; bx . instrprof_increment (fn_name , hash , num_counters , index) ; } CoverageKind :: VirtualCounter { .. } => { } } } }
    };
}

impl_271!();