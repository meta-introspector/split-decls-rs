macro_rules! deps {
    () => {
        GeneratedFileConflictsWithDirectory!();
        OutDirError!();
        InputFileWouldBeOverWritten!();
        TempsDirError!();
    };
}

macro_rules! write_dep_info {
    () => {
        deps!();
        pub fn write_dep_info (tcx : TyCtxt < '_ >) { let _ = tcx . resolver_for_lowering () ; let sess = tcx . sess ; let _timer = sess . timer ("write_dep_info") ; let crate_name = tcx . crate_name (LOCAL_CRATE) ; let outputs = tcx . output_filenames (()) ; let output_paths = generated_output_paths (tcx , & outputs , sess . io . output_file . is_some () , crate_name) ; if let Some (input_path) = sess . io . input . opt_path () { if sess . opts . will_create_output_file () { if output_contains_path (& output_paths , input_path) { sess . dcx () . emit_fatal (errors :: InputFileWouldBeOverWritten { path : input_path }) ; } if let Some (dir_path) = output_conflicts_with_dir (& output_paths) { sess . dcx () . emit_fatal (errors :: GeneratedFileConflictsWithDirectory { input_path , dir_path , }) ; } } } if let Some (ref dir) = sess . io . temps_dir { if fs :: create_dir_all (dir) . is_err () { sess . dcx () . emit_fatal (errors :: TempsDirError) ; } } write_out_deps (tcx , & outputs , & output_paths) ; let only_dep_info = sess . opts . output_types . contains_key (& OutputType :: DepInfo) && sess . opts . output_types . len () == 1 ; if ! only_dep_info { if let Some (ref dir) = sess . io . output_dir { if fs :: create_dir_all (dir) . is_err () { sess . dcx () . emit_fatal (errors :: OutDirError) ; } } } }
    };
}

write_dep_info!()