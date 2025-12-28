macro_rules! deps {
    () => {
        BenchmarkId!();
    };
}

macro_rules! base_dir_exists {
    () => {
        deps!();
        fn base_dir_exists (id : & BenchmarkId , baseline : & str , output_directory : & Path) -> bool { let mut base_dir = output_directory . to_owned () ; base_dir . push (id . as_directory_name ()) ; base_dir . push (baseline) ; base_dir . exists () }
    };
}

base_dir_exists!();