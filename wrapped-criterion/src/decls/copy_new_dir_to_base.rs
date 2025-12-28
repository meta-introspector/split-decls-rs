macro_rules! copy_new_dir_to_base {
    () => {
        fn copy_new_dir_to_base (id : & str , baseline : & str , output_directory : & Path) { let root_dir = Path :: new (output_directory) . join (id) ; let base_dir = root_dir . join (baseline) ; let new_dir = root_dir . join ("new") ; if ! new_dir . exists () { return ; } ; if ! base_dir . exists () { try_else_return ! (fs :: mkdirp (& base_dir)) ; } try_else_return ! (fs :: cp (& new_dir . join ("estimates.json") , & base_dir . join ("estimates.json"))) ; try_else_return ! (fs :: cp (& new_dir . join ("sample.json") , & base_dir . join ("sample.json"))) ; try_else_return ! (fs :: cp (& new_dir . join ("tukey.json") , & base_dir . join ("tukey.json"))) ; try_else_return ! (fs :: cp (& new_dir . join ("benchmark.json") , & base_dir . join ("benchmark.json"))) ; # [cfg (feature = "csv_output")] try_else_return ! (fs :: cp (& new_dir . join ("raw.csv") , & base_dir . join ("raw.csv"))) ; }
    };
}

copy_new_dir_to_base!();