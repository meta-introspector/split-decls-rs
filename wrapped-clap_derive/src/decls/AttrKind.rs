macro_rules! AttrKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq)] pub (crate) enum AttrKind { Clap , StructOpt , Command , Group , Arg , Value , }
    };
}

AttrKind!()