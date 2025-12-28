macro_rules! deps {
    () => {
        RPathConfig!();
    };
}

macro_rules! get_rpaths {
    () => {
        deps!();
        fn get_rpaths (config : & RPathConfig < '_ >) -> Vec < OsString > { debug ! ("output: {:?}" , config . out_filename . display ()) ; debug ! ("libs:") ; for libpath in config . libs { debug ! ("    {:?}" , libpath . display ()) ; } let rpaths = get_rpaths_relative_to_output (config) ; debug ! ("rpaths:") ; for rpath in & rpaths { debug ! ("    {:?}" , rpath) ; } minimize_rpaths (& rpaths) }
    };
}

get_rpaths!()