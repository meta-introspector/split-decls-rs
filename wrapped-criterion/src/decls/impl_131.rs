macro_rules! deps {
    () => {
        BenchmarkId!();
        IndividualBenchmark!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl IndividualBenchmark { fn from_id (output_directory : & Path , path_prefix : & str , id : & BenchmarkId ,) -> IndividualBenchmark { let mut regression_path = PathBuf :: from (output_directory) ; regression_path . push (id . as_directory_name ()) ; regression_path . push ("report") ; regression_path . push ("regression.svg") ; IndividualBenchmark { name : id . as_title () . to_owned () , path : format ! ("{}/{}" , path_prefix , id . as_directory_name ()) , regression_exists : regression_path . is_file () , } } }
    };
}

impl_131!()