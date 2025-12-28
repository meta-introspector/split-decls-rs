macro_rules! deps {
    () => {
        CodegenResults!();
        ArchiveBuilderBuilder!();
        Linker!();
        RlibArchiveBuildFailure!();
    };
}

macro_rules! add_static_crate {
    () => {
        deps!();
        fn add_static_crate (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , cnum : CrateNum , bundled_lib_file_names : & FxIndexSet < Symbol > ,) { let src = & codegen_results . crate_info . used_crate_source [& cnum] ; let cratepath = & src . rlib . as_ref () . unwrap () . 0 ; let mut link_upstream = | path : & Path | cmd . link_staticlib_by_path (& rehome_lib_path (sess , path) , false) ; if ! are_upstream_rust_objects_already_included (sess) || ignored_for_lto (sess , & codegen_results . crate_info , cnum) { link_upstream (cratepath) ; return ; } let dst = tmpdir . join (cratepath . file_name () . unwrap ()) ; let name = cratepath . file_name () . unwrap () . to_str () . unwrap () ; let name = & name [3 .. name . len () - 5] ; let bundled_lib_file_names = bundled_lib_file_names . clone () ; sess . prof . generic_activity_with_arg ("link_altering_rlib" , name) . run (| | { let canonical_name = name . replace ('-' , "_") ; let upstream_rust_objects_already_included = are_upstream_rust_objects_already_included (sess) ; let is_builtins = sess . target . no_builtins || ! codegen_results . crate_info . is_no_builtins . contains (& cnum) ; let mut archive = archive_builder_builder . new_archive_builder (sess) ; if let Err (error) = archive . add_archive (cratepath , Box :: new (move | f | { if f == METADATA_FILENAME { return true ; } let canonical = f . replace ('-' , "_") ; let is_rust_object = canonical . starts_with (& canonical_name) && looks_like_rust_object_file (f) ; if upstream_rust_objects_already_included && is_rust_object && is_builtins { return true ; } if bundled_lib_file_names . contains (& Symbol :: intern (f)) { return true ; } false }) ,) { sess . dcx () . emit_fatal (errors :: RlibArchiveBuildFailure { path : cratepath . clone () , error }) ; } if archive . build (& dst) { link_upstream (& dst) ; } }) ; }
    };
}

add_static_crate!();