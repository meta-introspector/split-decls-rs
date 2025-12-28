macro_rules! deps {
    () => {
        AttrKind!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl AttrKind { pub (crate) fn as_str (& self) -> & 'static str { match self { Self :: Clap => "clap" , Self :: StructOpt => "structopt" , Self :: Command => "command" , Self :: Group => "group" , Self :: Arg => "arg" , Self :: Value => "value" , } } }
    };
}

impl_11!()