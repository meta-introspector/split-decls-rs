macro_rules! deps {
    () => {
        AttrKind!();
        Sp!();
        Ty!();
        Kind!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Kind { pub (crate) fn name (& self) -> & 'static str { match self { Self :: Arg (_) => "arg" , Self :: Command (_) => "command" , Self :: Value => "value" , Self :: FromGlobal (_) => "from_global" , Self :: Subcommand (_) => "subcommand" , Self :: Flatten (_) => "flatten" , Self :: Skip (_ , _) => "skip" , Self :: ExternalSubcommand => "external_subcommand" , } } pub (crate) fn attr_kind (& self) -> AttrKind { match self { Self :: Arg (_) => AttrKind :: Arg , Self :: Command (_) => AttrKind :: Command , Self :: Value => AttrKind :: Value , Self :: FromGlobal (_) => AttrKind :: Arg , Self :: Subcommand (_) => AttrKind :: Command , Self :: Flatten (_) => AttrKind :: Command , Self :: Skip (_ , kind) => * kind , Self :: ExternalSubcommand => AttrKind :: Command , } } pub (crate) fn ty (& self) -> Option < & Sp < Ty > > { match self { Self :: Arg (ty) | Self :: Command (ty) | Self :: Flatten (ty) | Self :: FromGlobal (ty) | Self :: Subcommand (ty) => Some (ty) , Self :: Value | Self :: Skip (_ , _) | Self :: ExternalSubcommand => None , } } }
    };
}

impl_62!()