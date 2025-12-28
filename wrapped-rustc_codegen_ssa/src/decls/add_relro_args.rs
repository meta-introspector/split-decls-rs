macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! add_relro_args {
    () => {
        deps!();
        # [doc = " Add options making relocation sections in the produced ELF files read-only"] # [doc = " and suppressing lazy binding."] fn add_relro_args (cmd : & mut dyn Linker , sess : & Session) { match sess . opts . cg . relro_level . unwrap_or (sess . target . relro_level) { RelroLevel :: Full => cmd . full_relro () , RelroLevel :: Partial => cmd . partial_relro () , RelroLevel :: Off => cmd . no_relro () , RelroLevel :: None => { } } }
    };
}

add_relro_args!();