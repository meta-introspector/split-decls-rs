macro_rules! deps {
    () => {
        RPathConfig!();
    };
}

macro_rules! get_rpath_linker_args {
    () => {
        deps!();
        pub (super) fn get_rpath_linker_args (config : & RPathConfig < '_ >) -> Vec < OsString > { debug ! ("preparing the RPATH!") ; let rpaths = get_rpaths (config) ; let mut args = Vec :: with_capacity (rpaths . len () * 2) ; for rpath in rpaths { args . push ("-rpath" . into ()) ; args . push (rpath) ; } if config . linker_is_gnu { args . push ("--enable-new-dtags" . into ()) ; args . push ("-z" . into ()) ; args . push ("origin" . into ()) ; } args }
    };
}

get_rpath_linker_args!()