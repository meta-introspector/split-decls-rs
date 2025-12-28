macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! add_user_defined_link_args {
    () => {
        deps!();
        # [doc = " Add arbitrary \"user defined\" args defined from command line."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_user_defined_link_args (cmd : & mut dyn Linker , sess : & Session) { cmd . verbatim_args (& sess . opts . cg . link_args) ; }
    };
}

add_user_defined_link_args!()