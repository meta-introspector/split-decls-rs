macro_rules! deps {
    () => {
        AttrKind!();
        Sp!();
        AttrValue!();
        Ty!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [allow (clippy :: large_enum_variant)] # [derive (Clone)] pub (crate) enum Kind { Arg (Sp < Ty >) , Command (Sp < Ty >) , Value , FromGlobal (Sp < Ty >) , Subcommand (Sp < Ty >) , Flatten (Sp < Ty >) , Skip (Option < AttrValue > , AttrKind) , ExternalSubcommand , }
    };
}

Kind!();