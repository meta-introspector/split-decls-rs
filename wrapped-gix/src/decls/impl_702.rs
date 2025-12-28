macro_rules! deps {
    () => {
        Protocol!();
        Version!();
        Allow!();
        NameParameter!();
        Tree!();
    };
}

macro_rules! impl_702 {
    () => {
        deps!();
        impl Protocol { # [doc = " The `protocol.allow` key."] pub const ALLOW : Allow = Allow :: new_with_validate ("allow" , & config :: Tree :: PROTOCOL , validate :: Allow) ; # [doc = " The `protocol.version` key."] pub const VERSION : Version = Version :: new_with_validate ("version" , & config :: Tree :: PROTOCOL , validate :: Version) ; # [doc = " The `protocol.<name>` subsection"] pub const NAME_PARAMETER : NameParameter = NameParameter ; }
    };
}

impl_702!()