macro_rules! deps {
    () => {
        CrateInfo!();
    };
}

macro_rules! crate_info {
    () => {
        deps!();
        fn crate_info (data : & ide_db :: base_db :: BuiltCrateData , extra_data : & ide_db :: base_db :: ExtraCrateData ,) -> CrateInfo { let crate_name = crate_name (extra_data) ; let version = extra_data . version . clone () ; CrateInfo { name : crate_name , version , root_file_id : data . root_file_id } }
    };
}

crate_info!()