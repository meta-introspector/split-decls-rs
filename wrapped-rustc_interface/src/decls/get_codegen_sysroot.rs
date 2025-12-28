macro_rules! deps {
    () => {
        MakeBackendFn!();
    };
}

macro_rules! get_codegen_sysroot {
    () => {
        deps!();
        # [allow (rustc :: untranslatable_diagnostic)] fn get_codegen_sysroot (early_dcx : & EarlyDiagCtxt , sysroot : & Sysroot , backend_name : & str ,) -> MakeBackendFn { static LOADED : AtomicBool = AtomicBool :: new (false) ; assert ! (! LOADED . fetch_or (true , Ordering :: SeqCst) , "cannot load the default codegen backend twice") ; let target = host_tuple () ; let sysroot = sysroot . all_paths () . map (| sysroot | { filesearch :: make_target_lib_path (sysroot , target) . with_file_name ("codegen-backends") }) . find (| f | { info ! ("codegen backend candidate: {}" , f . display ()) ; f . exists () }) . unwrap_or_else (| | { let candidates = sysroot . all_paths () . map (| p | p . display () . to_string ()) . collect :: < Vec < _ > > () . join ("\n* ") ; let err = format ! ("failed to find a `codegen-backends` folder \
                           in the sysroot candidates:\n* {candidates}") ; early_dcx . early_fatal (err) ; }) ; info ! ("probing {} for a codegen backend" , sysroot . display ()) ; let d = sysroot . read_dir () . unwrap_or_else (| e | { let err = format ! ("failed to load default codegen backend, couldn't \
                           read `{}`: {}" , sysroot . display () , e) ; early_dcx . early_fatal (err) ; }) ; let mut file : Option < PathBuf > = None ; let expected_names = & [format ! ("rustc_codegen_{}-{}" , backend_name , env ! ("CFG_RELEASE")) , format ! ("rustc_codegen_{backend_name}") ,] ; for entry in d . filter_map (| e | e . ok ()) { let path = entry . path () ; let Some (filename) = path . file_name () . and_then (| s | s . to_str ()) else { continue } ; if ! (filename . starts_with (DLL_PREFIX) && filename . ends_with (DLL_SUFFIX)) { continue ; } let name = & filename [DLL_PREFIX . len () .. filename . len () - DLL_SUFFIX . len ()] ; if ! expected_names . iter () . any (| expected | expected == name) { continue ; } if let Some (ref prev) = file { let err = format ! ("duplicate codegen backends found\n\
                               first:  {}\n\
                               second: {}\n\
            " , prev . display () , path . display ()) ; early_dcx . early_fatal (err) ; } file = Some (path . clone ()) ; } match file { Some (ref s) => load_backend_from_dylib (early_dcx , s) , None => { let err = format ! ("unsupported builtin codegen backend `{backend_name}`") ; early_dcx . early_fatal (err) ; } } }
    };
}

get_codegen_sysroot!();