macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! GccLinker {
    () => {
        deps!();
        struct GccLinker < 'a > { cmd : Command , sess : & 'a Session , target_cpu : & 'a str , hinted_static : Option < bool > , is_ld : bool , is_gnu : bool , uses_lld : bool , }
    };
}

GccLinker!();