macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! verbatim_args {
    () => {
        deps!();
        # [doc = " Just pass the arguments to the linker as is."] # [doc = " It is assumed that they are correctly prepared in advance."] fn verbatim_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > > ,) -> & mut L { for arg in args { l . cmd () . arg (arg) ; } l }
    };
}

verbatim_args!()