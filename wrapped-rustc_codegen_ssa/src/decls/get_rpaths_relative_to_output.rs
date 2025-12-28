macro_rules! deps {
    () => {
        RPathConfig!();
    };
}

macro_rules! get_rpaths_relative_to_output {
    () => {
        deps!();
        fn get_rpaths_relative_to_output (config : & RPathConfig < '_ >) -> Vec < OsString > { config . libs . iter () . map (| a | get_rpath_relative_to_output (config , a)) . collect () }
    };
}

get_rpaths_relative_to_output!();