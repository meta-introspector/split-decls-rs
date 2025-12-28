macro_rules! crate_path {
    () => {
        fn crate_path (sess : & Session , crate_name : Symbol) -> PathBuf { let incr_dir = sess . opts . incremental . as_ref () . unwrap () . clone () ; let crate_types = collect_crate_types (sess , & []) ; let stable_crate_id = StableCrateId :: new (crate_name , crate_types . contains (& CrateType :: Executable) , sess . opts . cg . metadata . clone () , sess . cfg_version ,) ; let crate_name = format ! ("{crate_name}-{}" , stable_crate_id . as_u64 () . to_base_fixed_len (CASE_INSENSITIVE)) ; incr_dir . join (crate_name) }
    };
}

crate_path!()