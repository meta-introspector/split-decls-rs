macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! cc_args {
    () => {
        deps!();
        # [doc = " Arguments for the cc wrapper specifically."] # [doc = " Check that it's indeed a cc wrapper and pass verbatim."] fn cc_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > >) -> & mut L { assert ! (l . is_cc ()) ; verbatim_args (l , args) }
    };
}

cc_args!()