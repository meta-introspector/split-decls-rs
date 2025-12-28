macro_rules! deps {
    () => {
        Linker!();
        CodegenResults!();
        ArchiveBuilderBuilder!();
    };
}

macro_rules! add_native_libs_from_crate {
    () => {
        deps!();
        fn add_native_libs_from_crate (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , bundled_libs : & FxIndexSet < Symbol > , cnum : CrateNum , link_static : bool , link_dynamic : bool , link_output_kind : LinkOutputKind ,) { if ! sess . opts . unstable_opts . link_native_libraries { return ; } if link_static && cnum != LOCAL_CRATE && ! bundled_libs . is_empty () { let rlib = & codegen_results . crate_info . used_crate_source [& cnum] . rlib . as_ref () . unwrap () . 0 ; archive_builder_builder . extract_bundled_libs (rlib , tmpdir , bundled_libs) . unwrap_or_else (| e | sess . dcx () . emit_fatal (e)) ; } let native_libs = match cnum { LOCAL_CRATE => & codegen_results . crate_info . used_libraries , _ => & codegen_results . crate_info . native_libraries [& cnum] , } ; let mut last = (None , NativeLibKind :: Unspecified , false) ; for lib in native_libs { if ! relevant_lib (sess , lib) { continue ; } last = if (Some (lib . name) , lib . kind , lib . verbatim) == last { continue ; } else { (Some (lib . name) , lib . kind , lib . verbatim) } ; let name = lib . name . as_str () ; let verbatim = lib . verbatim ; match lib . kind { NativeLibKind :: Static { bundle , whole_archive } => { if link_static { let bundle = bundle . unwrap_or (true) ; let whole_archive = whole_archive == Some (true) ; if bundle && cnum != LOCAL_CRATE { if let Some (filename) = lib . filename { let path = tmpdir . join (filename . as_str ()) ; cmd . link_staticlib_by_path (& path , whole_archive) ; } } else { cmd . link_staticlib_by_name (name , verbatim , whole_archive) ; } } } NativeLibKind :: Dylib { as_needed } => { if link_dynamic { cmd . link_dylib_by_name (name , verbatim , as_needed . unwrap_or (true)) } } NativeLibKind :: Unspecified => { if ! link_output_kind . can_link_dylib () && ! sess . target . crt_static_allows_dylibs { if link_static { cmd . link_staticlib_by_name (name , verbatim , false) ; } } else if link_dynamic { cmd . link_dylib_by_name (name , verbatim , true) ; } } NativeLibKind :: Framework { as_needed } => { if link_dynamic { cmd . link_framework_by_name (name , verbatim , as_needed . unwrap_or (true)) } } NativeLibKind :: RawDylib => { } NativeLibKind :: WasmImportModule => { } NativeLibKind :: LinkArg => { if link_static { if verbatim { cmd . verbatim_arg (name) ; } else { cmd . link_arg (name) ; } } } } } }
    };
}

add_native_libs_from_crate!()