macro_rules! deps {
    () => {
        Pack!();
        Repository!();
        Error!();
    };
}

macro_rules! index_threads {
    () => {
        deps!();
        pub fn index_threads (repo : & Repository) -> Result < Option < usize > , Error > { Ok (repo . config . resolved . integer_filter (Pack :: THREADS , & mut repo . filter_config_section ()) . map (| threads | Pack :: THREADS . try_into_usize (threads)) . transpose () . with_leniency (repo . options . lenient_config) ?) }
    };
}

index_threads!()