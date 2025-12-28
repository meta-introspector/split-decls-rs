macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! link_or_cc_args {
    () => {
        deps!();
        # [doc = " Arguments supported by both underlying linker and cc wrapper, pass verbatim."] fn link_or_cc_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > > ,) -> & mut L { verbatim_args (l , args) }
    };
}

link_or_cc_args!();