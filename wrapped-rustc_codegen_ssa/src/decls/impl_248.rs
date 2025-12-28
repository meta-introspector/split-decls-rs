macro_rules! deps {
    () => {
        OngoingCodegen!();
        CguMessage!();
        ExtraBackendMethods!();
        CodegenResults!();
        Message!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < B : ExtraBackendMethods > OngoingCodegen < B > { pub fn join (self , sess : & Session) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { self . shared_emitter_main . check (sess , true) ; let compiled_modules = sess . time ("join_worker_thread" , | | match self . coordinator . join () { Ok (Ok (compiled_modules)) => compiled_modules , Ok (Err (())) => { sess . dcx () . abort_if_errors () ; panic ! ("expected abort due to worker thread errors") } Err (_) => { bug ! ("panic during codegen/LLVM phase") ; } }) ; sess . dcx () . abort_if_errors () ; let work_products = copy_all_cgu_workproducts_to_incr_comp_cache_dir (sess , & compiled_modules) ; produce_final_output_artifacts (sess , & compiled_modules , & self . output_filenames) ; if sess . codegen_units () . as_usize () == 1 && sess . opts . unstable_opts . time_llvm_passes { self . backend . print_pass_timings () } if sess . print_llvm_stats () { self . backend . print_statistics () } (CodegenResults { crate_info : self . crate_info , modules : compiled_modules . modules , allocator_module : compiled_modules . allocator_module , } , work_products ,) } pub (crate) fn codegen_finished (& self , tcx : TyCtxt < '_ >) { self . wait_for_signal_to_codegen_item () ; self . check_for_errors (tcx . sess) ; drop (self . coordinator . sender . send (Message :: CodegenComplete :: < B >)) ; } pub (crate) fn check_for_errors (& self , sess : & Session) { self . shared_emitter_main . check (sess , false) ; } pub (crate) fn wait_for_signal_to_codegen_item (& self) { match self . codegen_worker_receive . recv () { Ok (CguMessage) => { } Err (_) => { } } } }
    };
}

impl_248!()