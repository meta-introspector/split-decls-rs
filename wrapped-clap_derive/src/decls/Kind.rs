macro_rules! deps {
    () => {
        AttrValue!();
        Ty!();
        Sp!();
        AttrKind!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [allow (clippy :: large_enum_variant)] # [derive (Clone)] pub (crate) enum Kind { Arg (Sp < Ty >) , Command (Sp < Ty >) , Value , FromGlobal (Sp < Ty >) , Subcommand (Sp < Ty >) , Flatten (Sp < Ty >) , Skip (Option < AttrValue > , AttrKind) , ExternalSubcommand , }
    };
}

Kind!()